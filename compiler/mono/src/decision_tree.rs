use crate::exhaustive::{Ctor, RenderAs, TagId, Union};
use crate::ir::{
    BranchInfo, DestructType, Env, Expr, JoinPointId, Literal, Param, Pattern, Procs, Stmt, Wrapped,
};
use crate::layout::{Builtin, Layout, LayoutCache, UnionLayout};
use roc_collections::all::{MutMap, MutSet};
use roc_module::ident::TagName;
use roc_module::low_level::LowLevel;
use roc_module::symbol::Symbol;

/// COMPILE CASES

type Label = u64;
const RECORD_TAG_NAME: &str = "#Record";

/// Users of this module will mainly interact with this function. It takes
/// some normal branches and gives out a decision tree that has "labels" at all
/// the leafs and a dictionary that maps these "labels" to the code that should
/// run.
pub fn compile<'a>(raw_branches: Vec<(Guard<'a>, Pattern<'a>, u64)>) -> DecisionTree<'a> {
    let formatted = raw_branches
        .into_iter()
        .map(|(guard, pattern, index)| Branch {
            goal: index,
            patterns: vec![(Vec::new(), guard, pattern)],
        })
        .collect();

    to_decision_tree(formatted)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Guard<'a> {
    NoGuard,
    Guard {
        /// Symbol that stores a boolean
        /// when true this branch is picked, otherwise skipped
        symbol: Symbol,
        /// after assigning to symbol, the stmt jumps to this label
        id: JoinPointId,
        stmt: Stmt<'a>,
    },
}

impl<'a> Guard<'a> {
    fn is_none(&self) -> bool {
        self == &Guard::NoGuard
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DecisionTree<'a> {
    Match(Label),
    Decision {
        path: Vec<PathInstruction>,
        edges: Vec<(Test<'a>, DecisionTree<'a>)>,
        default: Option<Box<DecisionTree<'a>>>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Test<'a> {
    IsCtor {
        tag_id: u8,
        tag_name: TagName,
        union: crate::exhaustive::Union,
        arguments: Vec<(Pattern<'a>, Layout<'a>)>,
    },
    IsInt(i128),
    // float patterns are stored as u64 so they are comparable/hashable
    IsFloat(u64),
    IsStr(Box<str>),
    IsBit(bool),
    IsByte {
        tag_id: u8,
        num_alts: usize,
    },
    // A pattern that always succeeds (like `_`) can still have a guard
    Guarded {
        opt_test: Option<Box<Test<'a>>>,
        /// Symbol that stores a boolean
        /// when true this branch is picked, otherwise skipped
        symbol: Symbol,
        /// after assigning to symbol, the stmt jumps to this label
        id: JoinPointId,
        stmt: Stmt<'a>,
    },
}
use std::hash::{Hash, Hasher};
impl<'a> Hash for Test<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use Test::*;

        match self {
            IsCtor { tag_id, .. } => {
                state.write_u8(0);
                tag_id.hash(state);
                // The point of this custom implementation is to not hash the tag arguments
            }
            IsInt(v) => {
                state.write_u8(1);
                v.hash(state);
            }
            IsFloat(v) => {
                state.write_u8(2);
                v.hash(state);
            }
            IsStr(v) => {
                state.write_u8(3);
                v.hash(state);
            }
            IsBit(v) => {
                state.write_u8(4);
                v.hash(state);
            }
            IsByte { tag_id, num_alts } => {
                state.write_u8(5);
                tag_id.hash(state);
                num_alts.hash(state);
            }
            Guarded { opt_test: None, .. } => {
                state.write_u8(6);
            }
            Guarded {
                opt_test: Some(nested),
                ..
            } => {
                state.write_u8(7);
                nested.hash(state);
            }
        }
    }
}

// ACTUALLY BUILD DECISION TREES

#[derive(Clone, Debug, PartialEq)]
struct Branch<'a> {
    goal: Label,
    patterns: Vec<(Vec<PathInstruction>, Guard<'a>, Pattern<'a>)>,
}

fn to_decision_tree(raw_branches: Vec<Branch>) -> DecisionTree {
    let branches: Vec<_> = raw_branches.into_iter().map(flatten_patterns).collect();

    match check_for_match(&branches) {
        Some(goal) => DecisionTree::Match(goal),
        None => {
            // must clone here to release the borrow on `branches`
            let path = pick_path(&branches).clone();
            let (edges, fallback) = gather_edges(branches, &path);

            let mut decision_edges: Vec<_> = edges
                .into_iter()
                .map(|(a, b)| (a, to_decision_tree(b)))
                .collect();

            match (decision_edges.split_last_mut(), fallback.split_last()) {
                (Some(((_tag, decision_tree), rest)), None) if rest.is_empty() => {
                    // TODO remove clone
                    decision_tree.clone()
                }
                (_, None) => DecisionTree::Decision {
                    path: path.clone(),
                    edges: decision_edges,
                    default: None,
                },
                (None, Some(_)) => to_decision_tree(fallback),
                _ => DecisionTree::Decision {
                    path: path.clone(),
                    edges: decision_edges,
                    default: Some(Box::new(to_decision_tree(fallback))),
                },
            }
        }
    }
}

fn is_complete(tests: &[Test]) -> bool {
    let length = tests.len();
    debug_assert!(length > 0);
    match tests.last() {
        None => unreachable!("should never happen"),
        Some(v) => match v {
            Test::IsCtor { union, .. } => length == union.alternatives.len(),
            Test::IsByte { num_alts, .. } => length == *num_alts,
            Test::IsBit(_) => length == 2,
            Test::IsInt(_) => false,
            Test::IsFloat(_) => false,
            Test::IsStr(_) => false,
            Test::Guarded { .. } => false,
        },
    }
}

fn flatten_patterns(branch: Branch) -> Branch {
    let mut result = Vec::with_capacity(branch.patterns.len());

    for path_pattern in branch.patterns {
        flatten(path_pattern, &mut result);
    }

    Branch {
        goal: branch.goal,
        patterns: result,
    }
}

fn flatten<'a>(
    path_pattern: (Vec<PathInstruction>, Guard<'a>, Pattern<'a>),
    path_patterns: &mut Vec<(Vec<PathInstruction>, Guard<'a>, Pattern<'a>)>,
) {
    match path_pattern.2 {
        Pattern::AppliedTag {
            union,
            arguments,
            tag_id,
            tag_name,
            layout,
        } if union.alternatives.len() == 1
            && !matches!(
                layout,
                Layout::Union(UnionLayout::NullableWrapped { .. })
                    | Layout::Union(UnionLayout::NullableUnwrapped { .. })
            ) =>
        {
            // TODO ^ do we need to check that guard.is_none() here?

            let path = path_pattern.0;
            // Theory: unbox doesn't have any value for us, because one-element tag unions
            // don't store the tag anyway.
            if arguments.len() == 1 {
                // NOTE here elm will unbox, but we don't use that
                path_patterns.push((
                    path,
                    path_pattern.1.clone(),
                    Pattern::AppliedTag {
                        union,
                        arguments,
                        tag_id,
                        tag_name,
                        layout,
                    },
                ));
            } else {
                for (index, (arg_pattern, _)) in arguments.iter().enumerate() {
                    let mut new_path = path.clone();
                    new_path.push(PathInstruction {
                        index: index as u64,
                        tag_id,
                    });

                    flatten(
                        (
                            new_path,
                            // same guard here?
                            path_pattern.1.clone(),
                            arg_pattern.clone(),
                        ),
                        path_patterns,
                    );
                }
            }
        }

        _ => {
            path_patterns.push(path_pattern);
        }
    }
}

/// SUCCESSFULLY MATCH

/// If the first branch has no more "decision points" we can finally take that
/// path. If that is the case we give the resulting label and a mapping from free
/// variables to "how to get their value". So a pattern like (Just (x,_)) will give
/// us something like ("x" => value.0.0)
fn check_for_match(branches: &[Branch]) -> Option<Label> {
    match branches.get(0) {
        Some(Branch { goal, patterns })
            if patterns
                .iter()
                .all(|(_, guard, pattern)| guard.is_none() && !needs_tests(pattern)) =>
        {
            Some(*goal)
        }
        _ => None,
    }
}

/// GATHER OUTGOING EDGES

fn gather_edges<'a>(
    branches: Vec<Branch<'a>>,
    path: &[PathInstruction],
) -> (Vec<(Test<'a>, Vec<Branch<'a>>)>, Vec<Branch<'a>>) {
    let relevant_tests = tests_at_path(path, &branches);

    let check = is_complete(&relevant_tests);

    // TODO remove clone
    let all_edges = relevant_tests
        .into_iter()
        .map(|t| edges_for(path, branches.clone(), t))
        .collect();

    let fallbacks = if check {
        vec![]
    } else {
        branches
            .into_iter()
            .filter(|b| is_irrelevant_to(path, b))
            .collect()
    };

    (all_edges, fallbacks)
}

/// FIND RELEVANT TESTS

fn tests_at_path<'a>(selected_path: &[PathInstruction], branches: &[Branch<'a>]) -> Vec<Test<'a>> {
    // NOTE the ordering of the result is important!

    let mut all_tests = Vec::new();

    for branch in branches {
        test_at_path(selected_path, branch, &mut all_tests);
    }

    // The rust HashMap also uses equality, here we really want to use the custom hash function
    // defined on Test to determine whether a test is unique. So we have to do the hashing
    // explicitly

    use std::collections::hash_map::DefaultHasher;

    let mut visited = MutSet::default();
    let mut unique = Vec::new();

    for test in all_tests {
        let hash = {
            let mut hasher = DefaultHasher::new();
            test.hash(&mut hasher);
            hasher.finish()
        };

        if !visited.contains(&hash) {
            visited.insert(hash);
            unique.push(test);
        }
    }

    unique
}

fn test_at_path<'a>(
    selected_path: &[PathInstruction],
    branch: &Branch<'a>,
    all_tests: &mut Vec<Test<'a>>,
) {
    use Pattern::*;
    use Test::*;

    match branch
        .patterns
        .iter()
        .find(|(path, _, _)| path == selected_path)
    {
        None => {}
        Some((_, guard, pattern)) => {
            let guarded = |test| {
                if let Guard::Guard { symbol, id, stmt } = guard {
                    Guarded {
                        opt_test: Some(Box::new(test)),
                        stmt: stmt.clone(),
                        symbol: *symbol,
                        id: *id,
                    }
                } else {
                    test
                }
            };

            match pattern {
                // TODO use guard!
                Identifier(_) | Underscore => {
                    if let Guard::Guard { symbol, id, stmt } = guard {
                        all_tests.push(Guarded {
                            opt_test: None,
                            stmt: stmt.clone(),
                            symbol: *symbol,
                            id: *id,
                        });
                    }
                }

                RecordDestructure(destructs, _) => {
                    // not rendered, so pick the easiest
                    let union = Union {
                        render_as: RenderAs::Tag,
                        alternatives: vec![Ctor {
                            tag_id: TagId(0),
                            name: TagName::Global(RECORD_TAG_NAME.into()),
                            arity: destructs.len(),
                        }],
                    };

                    let mut arguments = std::vec::Vec::new();

                    for destruct in destructs {
                        match &destruct.typ {
                            DestructType::Guard(guard) => {
                                arguments.push((guard.clone(), destruct.layout));
                            }
                            DestructType::Required(_) => {
                                arguments.push((Pattern::Underscore, destruct.layout));
                            }
                        }
                    }

                    all_tests.push(IsCtor {
                        tag_id: 0,
                        tag_name: TagName::Global(RECORD_TAG_NAME.into()),
                        union,
                        arguments,
                    });
                }

                AppliedTag {
                    tag_name,
                    tag_id,
                    arguments,
                    union,
                    ..
                } => {
                    all_tests.push(IsCtor {
                        tag_id: *tag_id,
                        tag_name: tag_name.clone(),
                        union: union.clone(),
                        arguments: arguments.to_vec(),
                    });
                }
                BitLiteral { value, .. } => {
                    all_tests.push(IsBit(*value));
                }
                EnumLiteral { tag_id, union, .. } => {
                    all_tests.push(IsByte {
                        tag_id: *tag_id,
                        num_alts: union.alternatives.len(),
                    });
                }
                IntLiteral(v) => {
                    all_tests.push(guarded(IsInt(*v)));
                }
                FloatLiteral(v) => {
                    all_tests.push(IsFloat(*v));
                }
                StrLiteral(v) => {
                    all_tests.push(IsStr(v.clone()));
                }
            };
        }
    }
}

/// BUILD EDGES

fn edges_for<'a>(
    path: &[PathInstruction],
    branches: Vec<Branch<'a>>,
    test: Test<'a>,
) -> (Test<'a>, Vec<Branch<'a>>) {
    let mut new_branches = Vec::new();

    for branch in branches.iter() {
        to_relevant_branch(&test, path, branch, &mut new_branches);
    }

    (test, new_branches)
}

fn to_relevant_branch<'a>(
    test: &Test<'a>,
    path: &[PathInstruction],
    branch: &Branch<'a>,
    new_branches: &mut Vec<Branch<'a>>,
) {
    // TODO remove clone
    match extract(path, branch.patterns.clone()) {
        Extract::NotFound => {
            new_branches.push(branch.clone());
        }
        Extract::Found {
            start,
            found_pattern: (guard, pattern),
            end,
        } => {
            let actual_test = match test {
                Test::Guarded {
                    opt_test: Some(box_test),
                    ..
                } => box_test,
                _ => test,
            };

            if let Some(mut new_branch) =
                to_relevant_branch_help(actual_test, path, start, end, branch, guard, pattern)
            {
                // guards can/should only occur at the top level. When we recurse on these
                // branches, the guard is not relevant any more. Not setthing the guard to None
                // leads to infinite recursion.
                new_branch.patterns.iter_mut().for_each(|(_, guard, _)| {
                    *guard = Guard::NoGuard;
                });

                new_branches.push(new_branch);
            }
        }
    }
}

fn to_relevant_branch_help<'a>(
    test: &Test<'a>,
    path: &[PathInstruction],
    mut start: Vec<(Vec<PathInstruction>, Guard<'a>, Pattern<'a>)>,
    end: Vec<(Vec<PathInstruction>, Guard<'a>, Pattern<'a>)>,
    branch: &Branch<'a>,
    guard: Guard<'a>,
    pattern: Pattern<'a>,
) -> Option<Branch<'a>> {
    use Pattern::*;
    use Test::*;

    match pattern {
        Identifier(_) | Underscore => Some(branch.clone()),

        RecordDestructure(destructs, _) => match test {
            IsCtor {
                tag_name: test_name,
                tag_id,
                ..
            } => {
                debug_assert!(test_name == &TagName::Global(RECORD_TAG_NAME.into()));
                let sub_positions = destructs.into_iter().enumerate().map(|(index, destruct)| {
                    let pattern = match destruct.typ {
                        DestructType::Guard(guard) => guard.clone(),
                        DestructType::Required(_) => Pattern::Underscore,
                    };

                    let mut new_path = path.to_vec();
                    new_path.push(PathInstruction {
                        index: index as u64,
                        tag_id: *tag_id,
                    });

                    (new_path, Guard::NoGuard, pattern)
                });
                start.extend(sub_positions);
                start.extend(end);

                Some(Branch {
                    goal: branch.goal,
                    patterns: start,
                })
            }
            _ => None,
        },

        AppliedTag {
            tag_name,
            tag_id,
            arguments,
            layout,
            ..
        } => {
            match test {
                IsCtor {
                    tag_name: test_name,
                    tag_id: test_id,
                    ..
                } if &tag_name == test_name => {
                    debug_assert_eq!(tag_id, *test_id);

                    // the test matches the constructor of this pattern

                    match Wrapped::opt_from_layout(&layout) {
                        None => todo!(),
                        Some(wrapped) => {
                            match wrapped {
                                Wrapped::SingleElementRecord => {
                                    // Theory: Unbox doesn't have any value for us
                                    debug_assert_eq!(arguments.len(), 1);
                                    let arg = arguments[0].clone();
                                    {
                                        // NOTE here elm unboxes, but we ignore that
                                        // Path::Unbox(Box::new(path.clone()))
                                        start.push((path.to_vec(), guard, arg.0));
                                        start.extend(end);
                                    }
                                }
                                Wrapped::RecordOrSingleTagUnion => {
                                    let sub_positions = arguments.into_iter().enumerate().map(
                                        |(index, (pattern, _))| {
                                            let mut new_path = path.to_vec();
                                            new_path.push(PathInstruction {
                                                index: index as u64,
                                                tag_id,
                                            });
                                            (new_path, Guard::NoGuard, pattern)
                                        },
                                    );
                                    start.extend(sub_positions);
                                    start.extend(end);
                                }
                                Wrapped::MultiTagUnion => {
                                    let sub_positions = arguments.into_iter().enumerate().map(
                                        |(index, (pattern, _))| {
                                            let mut new_path = path.to_vec();
                                            new_path.push(PathInstruction {
                                                index: 1 + index as u64,
                                                tag_id,
                                            });
                                            (new_path, Guard::NoGuard, pattern)
                                        },
                                    );
                                    start.extend(sub_positions);
                                    start.extend(end);
                                }
                                Wrapped::EmptyRecord => todo!(),
                            }

                            Some(Branch {
                                goal: branch.goal,
                                patterns: start,
                            })
                        }
                    }
                }
                _ => None,
            }
        }
        StrLiteral(string) => match test {
            IsStr(test_str) if string == *test_str => {
                start.extend(end);
                Some(Branch {
                    goal: branch.goal,
                    patterns: start,
                })
            }
            _ => None,
        },

        IntLiteral(int) => match test {
            IsInt(is_int) if int == *is_int => {
                start.extend(end);
                Some(Branch {
                    goal: branch.goal,
                    patterns: start,
                })
            }
            _ => None,
        },

        FloatLiteral(float) => match test {
            IsFloat(test_float) if float == *test_float => {
                start.extend(end);
                Some(Branch {
                    goal: branch.goal,
                    patterns: start,
                })
            }
            _ => None,
        },

        BitLiteral { value: bit, .. } => match test {
            IsBit(test_bit) if bit == *test_bit => {
                start.extend(end);
                Some(Branch {
                    goal: branch.goal,
                    patterns: start,
                })
            }
            _ => None,
        },

        EnumLiteral { tag_id, .. } => match test {
            IsByte {
                tag_id: test_id, ..
            } if tag_id == *test_id => {
                start.extend(end);
                Some(Branch {
                    goal: branch.goal,
                    patterns: start,
                })
            }

            _ => None,
        },
    }
}

enum Extract<'a> {
    NotFound,
    Found {
        start: Vec<(Vec<PathInstruction>, Guard<'a>, Pattern<'a>)>,
        found_pattern: (Guard<'a>, Pattern<'a>),
        end: Vec<(Vec<PathInstruction>, Guard<'a>, Pattern<'a>)>,
    },
}

fn extract<'a>(
    selected_path: &[PathInstruction],
    path_patterns: Vec<(Vec<PathInstruction>, Guard<'a>, Pattern<'a>)>,
) -> Extract<'a> {
    let mut start = Vec::new();

    // TODO potential ordering problem
    let mut it = path_patterns.into_iter();
    while let Some(current) = it.next() {
        if current.0 == selected_path {
            return Extract::Found {
                start,
                found_pattern: (current.1, current.2),
                end: it.collect::<Vec<_>>(),
            };
        } else {
            start.push(current);
        }
    }

    Extract::NotFound
}

/// FIND IRRELEVANT BRANCHES

fn is_irrelevant_to<'a>(selected_path: &[PathInstruction], branch: &Branch<'a>) -> bool {
    match branch
        .patterns
        .iter()
        .find(|(path, _, _)| path == selected_path)
    {
        None => true,
        Some((_, guard, pattern)) => guard.is_none() && !needs_tests(pattern),
    }
}

fn needs_tests(pattern: &Pattern) -> bool {
    use Pattern::*;

    match pattern {
        Identifier(_) | Underscore => false,

        RecordDestructure(_, _)
        | AppliedTag { .. }
        | BitLiteral { .. }
        | EnumLiteral { .. }
        | IntLiteral(_)
        | FloatLiteral(_)
        | StrLiteral(_) => true,
    }
}

/// PICK A PATH

fn pick_path<'a>(branches: &'a [Branch]) -> &'a Vec<PathInstruction> {
    let mut all_paths = Vec::with_capacity(branches.len());

    // is choice path
    for branch in branches {
        for (path, guard, pattern) in &branch.patterns {
            if !guard.is_none() || needs_tests(&pattern) {
                all_paths.push(path);
            } else {
                // do nothing
            }
        }
    }

    let mut by_small_defaults = bests_by_small_defaults(branches, all_paths.into_iter());

    if by_small_defaults.len() == 1 {
        by_small_defaults.remove(0)
    } else {
        debug_assert!(!by_small_defaults.is_empty());
        let mut result = bests_by_small_branching_factor(branches, by_small_defaults.into_iter());

        match result.pop() {
            None => unreachable!("bests_by will always return at least one value in the vec"),
            Some(path) => path,
        }
    }
}

fn bests_by_small_branching_factor<'a, I>(
    branches: &[Branch],
    mut all_paths: I,
) -> Vec<&'a Vec<PathInstruction>>
where
    I: Iterator<Item = &'a Vec<PathInstruction>>,
{
    match all_paths.next() {
        None => panic!("Cannot choose the best of zero paths. This should never happen."),
        Some(first_path) => {
            let mut min_weight = small_branching_factor(branches, first_path);
            let mut min_paths = vec![first_path];

            for path in all_paths {
                let weight = small_branching_factor(branches, path);

                use std::cmp::Ordering;
                match weight.cmp(&min_weight) {
                    Ordering::Equal => {
                        min_paths.push(path);
                    }
                    Ordering::Less => {
                        min_weight = weight;
                        min_paths.clear();
                        min_paths.push(path);
                    }
                    Ordering::Greater => {}
                }
            }

            min_paths
        }
    }
}

fn bests_by_small_defaults<'a, I>(
    branches: &[Branch],
    mut all_paths: I,
) -> Vec<&'a Vec<PathInstruction>>
where
    I: Iterator<Item = &'a Vec<PathInstruction>>,
{
    match all_paths.next() {
        None => panic!("Cannot choose the best of zero paths. This should never happen."),
        Some(first_path) => {
            let mut min_weight = small_defaults(branches, first_path);
            let mut min_paths = vec![first_path];

            for path in all_paths {
                let weight = small_defaults(branches, &path);

                use std::cmp::Ordering;
                match weight.cmp(&min_weight) {
                    Ordering::Equal => {
                        min_paths.push(path);
                    }
                    Ordering::Less => {
                        min_weight = weight;
                        min_paths.clear();
                        min_paths.push(path);
                    }
                    Ordering::Greater => {}
                }
            }

            min_paths
        }
    }
}

/// PATH PICKING HEURISTICS

fn small_defaults(branches: &[Branch], path: &[PathInstruction]) -> usize {
    branches
        .iter()
        .filter(|b| is_irrelevant_to(path, b))
        .map(|_| 1)
        .sum()
}

fn small_branching_factor(branches: &[Branch], path: &[PathInstruction]) -> usize {
    let (edges, fallback) = gather_edges(branches.to_vec(), path);

    edges.len() + (if fallback.is_empty() { 0 } else { 1 })
}

#[derive(Clone, Debug, PartialEq)]
enum Decider<'a, T> {
    Leaf(T),
    Chain {
        test_chain: Vec<(Vec<PathInstruction>, Test<'a>)>,
        success: Box<Decider<'a, T>>,
        failure: Box<Decider<'a, T>>,
    },
    FanOut {
        path: Vec<PathInstruction>,
        tests: Vec<(Test<'a>, Decider<'a, T>)>,
        fallback: Box<Decider<'a, T>>,
    },
}

#[derive(Clone, Debug, PartialEq)]
enum Choice<'a> {
    Inline(Stmt<'a>),
    Jump(Label),
}

type StoresVec<'a> = bumpalo::collections::Vec<'a, (Symbol, Layout<'a>, Expr<'a>)>;

pub fn optimize_when<'a>(
    env: &mut Env<'a, '_>,
    procs: &mut Procs<'a>,
    layout_cache: &mut LayoutCache<'a>,
    cond_symbol: Symbol,
    cond_layout: Layout<'a>,
    ret_layout: Layout<'a>,
    opt_branches: bumpalo::collections::Vec<'a, (Pattern<'a>, Guard<'a>, Stmt<'a>)>,
) -> Stmt<'a> {
    let (patterns, _indexed_branches) = opt_branches
        .into_iter()
        .enumerate()
        .map(|(index, (pattern, guard, branch))| {
            ((guard, pattern, index as u64), (index as u64, branch))
        })
        .unzip();

    let indexed_branches: Vec<(u64, Stmt<'a>)> = _indexed_branches;

    let decision_tree = compile(patterns);
    let decider = tree_to_decider(decision_tree);

    // for each target (branch body), count in how many ways it can be reached
    let mut target_counts = bumpalo::vec![in env.arena; 0; indexed_branches.len()];
    count_targets(&mut target_counts, &decider);

    let mut choices = MutMap::default();
    let mut jumps = Vec::new();

    for (index, branch) in indexed_branches.into_iter() {
        let ((branch_index, choice), opt_jump) = create_choices(&target_counts, index, branch);

        if let Some((index, body)) = opt_jump {
            let id = JoinPointId(env.unique_symbol());
            jumps.push((index, id, body));
        }

        choices.insert(branch_index, choice);
    }

    let choice_decider = insert_choices(&choices, decider);

    let mut stmt = decide_to_branching(
        env,
        procs,
        layout_cache,
        cond_symbol,
        cond_layout,
        ret_layout,
        choice_decider,
        &jumps,
    );

    for (_, id, body) in jumps.into_iter() {
        stmt = Stmt::Join {
            id,
            parameters: &[],
            continuation: env.arena.alloc(body),
            remainder: env.arena.alloc(stmt),
        };
    }

    stmt
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PathInstruction {
    index: u64,
    tag_id: u8,
}

fn path_to_expr_help<'a>(
    env: &mut Env<'a, '_>,
    mut symbol: Symbol,
    path: &[PathInstruction],
    mut layout: Layout<'a>,
) -> (Symbol, StoresVec<'a>, Layout<'a>) {
    let mut stores = bumpalo::collections::Vec::new_in(env.arena);

    // let instructions = reverse_path(path);
    let instructions = path;
    let mut it = instructions.iter().peekable();

    while let Some(PathInstruction { index, tag_id }) = it.next() {
        match Wrapped::opt_from_layout(&layout) {
            None => {
                // this MUST be an index into a single-element (hence unwrapped) record

                debug_assert_eq!(*index, 0, "{:?}", &layout);
                debug_assert_eq!(*tag_id, 0);
                debug_assert!(it.peek().is_none());

                let field_layouts = vec![layout];

                debug_assert!(*index < field_layouts.len() as u64);

                debug_assert_eq!(field_layouts.len(), 1);

                let inner_expr = Expr::AccessAtIndex {
                    index: *index,
                    field_layouts: env.arena.alloc(field_layouts),
                    structure: symbol,
                    wrapped: Wrapped::SingleElementRecord,
                };

                symbol = env.unique_symbol();
                let inner_layout = layout;
                stores.push((symbol, inner_layout, inner_expr));

                break;
            }
            Some(wrapped) => {
                let index = *index;

                let field_layouts = match &layout {
                    Layout::Union(variant) => {
                        use UnionLayout::*;

                        match variant {
                            NonRecursive(layouts) | Recursive(layouts) => layouts[*tag_id as usize],
                            NonNullableUnwrapped(fields) => {
                                debug_assert_eq!(*tag_id, 0);
                                fields
                            }
                            NullableWrapped {
                                nullable_id,
                                other_tags: layouts,
                                ..
                            } => {
                                use std::cmp::Ordering;
                                match (*tag_id as usize).cmp(&(*nullable_id as usize)) {
                                    Ordering::Equal => {
                                        // the nullable tag is going to pretend it stores a tag id
                                        &*env
                                            .arena
                                            .alloc([Layout::Builtin(crate::layout::TAG_SIZE)])
                                    }
                                    Ordering::Less => layouts[*tag_id as usize],
                                    Ordering::Greater => layouts[*tag_id as usize - 1],
                                }
                            }
                            NullableUnwrapped {
                                nullable_id,
                                other_fields,
                            } => {
                                let tag_id = *tag_id != 0;

                                if tag_id == *nullable_id {
                                    // the nullable tag has no fields; we can only lookup its tag id
                                    debug_assert_eq!(index, 0);

                                    // the nullable tag is going to pretend it stores a tag id
                                    &*env.arena.alloc([Layout::Builtin(crate::layout::TAG_SIZE)])
                                } else {
                                    *other_fields
                                }
                            }
                        }
                    }

                    Layout::Struct(layouts) => layouts,
                    other => env.arena.alloc([*other]),
                };

                debug_assert!(
                    index < field_layouts.len() as u64,
                    "{} {:?} {:?} {:?}",
                    index,
                    field_layouts,
                    &layout,
                    tag_id,
                );

                let inner_layout = match &field_layouts[index as usize] {
                    Layout::RecursivePointer => layout,
                    other => *other,
                };

                let inner_expr = Expr::AccessAtIndex {
                    index,
                    field_layouts,
                    structure: symbol,
                    wrapped,
                };

                symbol = env.unique_symbol();
                stores.push((symbol, inner_layout, inner_expr));

                layout = inner_layout;
            }
        }
    }

    (symbol, stores, layout)
}

fn test_to_equality<'a>(
    env: &mut Env<'a, '_>,
    cond_symbol: Symbol,
    cond_layout: &Layout<'a>,
    path: &[PathInstruction],
    test: Test<'a>,
) -> (
    StoresVec<'a>,
    Symbol,
    Symbol,
    Layout<'a>,
    Option<ConstructorKnown<'a>>,
) {
    let (rhs_symbol, mut stores, _layout) =
        path_to_expr_help(env, cond_symbol, &path, *cond_layout);

    match test {
        Test::IsCtor {
            tag_id,
            union,
            arguments,
            ..
        } => {
            let path_symbol = rhs_symbol;
            // the IsCtor check should never be generated for tag unions of size 1
            // (e.g. record pattern guard matches)
            debug_assert!(union.alternatives.len() > 1);

            let lhs = Expr::Literal(Literal::Int(tag_id as i128));

            let mut field_layouts =
                bumpalo::collections::Vec::with_capacity_in(arguments.len(), env.arena);

            // add the tag discriminant
            field_layouts.push(Layout::Builtin(Builtin::Int64));

            for (_, layout) in arguments {
                field_layouts.push(layout);
            }
            let field_layouts = field_layouts.into_bump_slice();

            let rhs = Expr::AccessAtIndex {
                index: 0,
                field_layouts,
                structure: path_symbol,
                wrapped: Wrapped::MultiTagUnion,
            };

            let lhs_symbol = env.unique_symbol();
            let rhs_symbol = env.unique_symbol();

            stores.push((lhs_symbol, Layout::Builtin(Builtin::Int64), lhs));
            stores.push((rhs_symbol, Layout::Builtin(Builtin::Int64), rhs));

            (
                stores,
                lhs_symbol,
                rhs_symbol,
                Layout::Builtin(Builtin::Int64),
                Some(ConstructorKnown::OnlyPass {
                    scrutinee: path_symbol,
                    layout: *cond_layout,
                    tag_id,
                }),
            )
        }
        Test::IsInt(test_int) => {
            // TODO don't downcast i128 here
            debug_assert!(test_int <= i64::MAX as i128);
            let lhs = Expr::Literal(Literal::Int(test_int as i128));
            let lhs_symbol = env.unique_symbol();
            stores.push((lhs_symbol, Layout::Builtin(Builtin::Int64), lhs));

            (
                stores,
                lhs_symbol,
                rhs_symbol,
                Layout::Builtin(Builtin::Int64),
                None,
            )
        }

        Test::IsFloat(test_int) => {
            // TODO maybe we can actually use i64 comparison here?
            let test_float = f64::from_bits(test_int as u64);
            let lhs = Expr::Literal(Literal::Float(test_float));
            let lhs_symbol = env.unique_symbol();
            stores.push((lhs_symbol, Layout::Builtin(Builtin::Float64), lhs));

            (
                stores,
                lhs_symbol,
                rhs_symbol,
                Layout::Builtin(Builtin::Float64),
                None,
            )
        }

        Test::IsByte {
            tag_id: test_byte, ..
        } => {
            let lhs = Expr::Literal(Literal::Byte(test_byte));
            let lhs_symbol = env.unique_symbol();
            stores.push((lhs_symbol, Layout::Builtin(Builtin::Int8), lhs));

            (
                stores,
                lhs_symbol,
                rhs_symbol,
                Layout::Builtin(Builtin::Int8),
                None,
            )
        }

        Test::IsBit(test_bit) => {
            let lhs = Expr::Literal(Literal::Bool(test_bit));
            let lhs_symbol = env.unique_symbol();
            stores.push((lhs_symbol, Layout::Builtin(Builtin::Int1), lhs));

            (
                stores,
                lhs_symbol,
                rhs_symbol,
                Layout::Builtin(Builtin::Int1),
                None,
            )
        }

        Test::IsStr(test_str) => {
            let lhs = Expr::Literal(Literal::Str(env.arena.alloc(test_str)));
            let lhs_symbol = env.unique_symbol();

            stores.push((lhs_symbol, Layout::Builtin(Builtin::Str), lhs));

            (
                stores,
                lhs_symbol,
                rhs_symbol,
                Layout::Builtin(Builtin::Str),
                None,
            )
        }

        Test::Guarded { .. } => unreachable!("should be handled elsewhere"),
    }
}

type Tests<'a> = std::vec::Vec<(
    bumpalo::collections::Vec<'a, (Symbol, Layout<'a>, Expr<'a>)>,
    Symbol,
    Symbol,
    Layout<'a>,
    Option<ConstructorKnown<'a>>,
)>;

fn stores_and_condition<'a>(
    env: &mut Env<'a, '_>,
    cond_symbol: Symbol,
    cond_layout: &Layout<'a>,
    test_chain: Vec<(Vec<PathInstruction>, Test<'a>)>,
) -> (Tests<'a>, Option<(Symbol, JoinPointId, Stmt<'a>)>) {
    let mut tests: Tests = Vec::with_capacity(test_chain.len());

    let mut guard = None;

    // Assumption: there is at most 1 guard, and it is the outer layer.
    for (path, test) in test_chain {
        match test {
            Test::Guarded {
                opt_test,
                id,
                symbol,
                stmt,
            } => {
                if let Some(nested) = opt_test {
                    tests.push(test_to_equality(
                        env,
                        cond_symbol,
                        &cond_layout,
                        &path,
                        *nested,
                    ));
                }

                // let (stores, rhs_symbol) = path_to_expr(env, cond_symbol, &path, &cond_layout);

                guard = Some((symbol, id, stmt));
            }

            _ => tests.push(test_to_equality(
                env,
                cond_symbol,
                &cond_layout,
                &path,
                test,
            )),
        }
    }

    (tests, guard)
}

fn compile_guard<'a>(
    env: &mut Env<'a, '_>,
    ret_layout: Layout<'a>,
    id: JoinPointId,
    stmt: &'a Stmt<'a>,
    fail: &'a Stmt<'a>,
    mut cond: Stmt<'a>,
) -> Stmt<'a> {
    // the guard is the final thing that we check, so needs to be layered on first!
    let test_symbol = env.unique_symbol();
    let arena = env.arena;

    cond = crate::ir::cond(
        env,
        test_symbol,
        Layout::Builtin(Builtin::Int1),
        cond,
        fail.clone(),
        ret_layout,
    );

    // calculate the guard value
    let param = Param {
        symbol: test_symbol,
        layout: Layout::Builtin(Builtin::Int1),
        borrow: false,
    };

    Stmt::Join {
        id,
        parameters: arena.alloc([param]),
        remainder: stmt,
        continuation: arena.alloc(cond),
    }
}

fn compile_test<'a>(
    env: &mut Env<'a, '_>,
    ret_layout: Layout<'a>,
    stores: bumpalo::collections::Vec<'a, (Symbol, Layout<'a>, Expr<'a>)>,
    lhs: Symbol,
    rhs: Symbol,
    fail: &'a Stmt<'a>,
    cond: Stmt<'a>,
) -> Stmt<'a> {
    compile_test_help(
        env,
        ConstructorKnown::Neither,
        ret_layout,
        stores,
        lhs,
        rhs,
        fail,
        cond,
    )
}

#[allow(clippy::too_many_arguments)]
fn compile_test_help<'a>(
    env: &mut Env<'a, '_>,
    branch_info: ConstructorKnown<'a>,
    ret_layout: Layout<'a>,
    stores: bumpalo::collections::Vec<'a, (Symbol, Layout<'a>, Expr<'a>)>,
    lhs: Symbol,
    rhs: Symbol,
    fail: &'a Stmt<'a>,
    mut cond: Stmt<'a>,
) -> Stmt<'a> {
    // if test_symbol then cond else fail
    let test_symbol = env.unique_symbol();
    let arena = env.arena;

    let (pass_info, fail_info) = {
        use ConstructorKnown::*;
        match branch_info {
            Both {
                scrutinee,
                layout,
                pass,
                fail,
            } => {
                let pass_info = BranchInfo::Constructor {
                    scrutinee,
                    layout,
                    tag_id: pass,
                };
                let fail_info = BranchInfo::Constructor {
                    scrutinee,
                    layout,
                    tag_id: fail,
                };

                (pass_info, fail_info)
            }

            OnlyPass {
                scrutinee,
                layout,
                tag_id,
            } => {
                let pass_info = BranchInfo::Constructor {
                    scrutinee,
                    layout,
                    tag_id,
                };

                (pass_info, BranchInfo::None)
            }

            Neither => (BranchInfo::None, BranchInfo::None),
        }
    };

    let branches = env.arena.alloc([(1u64, pass_info, cond)]);
    let default_branch = (fail_info, &*env.arena.alloc(fail.clone()));

    cond = Stmt::Switch {
        cond_symbol: test_symbol,
        cond_layout: Layout::Builtin(Builtin::Int1),
        ret_layout,
        branches,
        default_branch,
    };

    let op = LowLevel::Eq;
    let test = Expr::Call(crate::ir::Call {
        call_type: crate::ir::CallType::LowLevel {
            op,
            update_mode: env.next_update_mode_id(),
        },
        arguments: arena.alloc([lhs, rhs]),
    });

    // write to the test symbol
    cond = Stmt::Let(
        test_symbol,
        test,
        Layout::Builtin(Builtin::Int1),
        arena.alloc(cond),
    );

    // stores are in top-to-bottom order, so we have to add them in reverse
    for (symbol, layout, expr) in stores.into_iter().rev() {
        cond = Stmt::Let(symbol, expr, layout, arena.alloc(cond));
    }

    cond
}

fn compile_tests<'a>(
    env: &mut Env<'a, '_>,
    ret_layout: Layout<'a>,
    tests: Tests<'a>,
    opt_guard: Option<(Symbol, JoinPointId, Stmt<'a>)>,
    fail: &'a Stmt<'a>,
    mut cond: Stmt<'a>,
) -> Stmt<'a> {
    let arena = env.arena;

    // the guard is the final thing that we check, so needs to be layered on first!
    if let Some((_, id, stmt)) = opt_guard {
        cond = compile_guard(env, ret_layout, id, arena.alloc(stmt), fail, cond);
    }

    for (new_stores, lhs, rhs, _layout, opt_constructor_info) in tests.into_iter() {
        match opt_constructor_info {
            None => {
                cond = compile_test(env, ret_layout, new_stores, lhs, rhs, fail, cond);
            }
            Some(cinfo) => {
                cond = compile_test_help(env, cinfo, ret_layout, new_stores, lhs, rhs, fail, cond);
            }
        }
    }
    cond
}

#[derive(Debug)]
enum ConstructorKnown<'a> {
    Both {
        scrutinee: Symbol,
        layout: Layout<'a>,
        pass: u8,
        fail: u8,
    },
    OnlyPass {
        scrutinee: Symbol,
        layout: Layout<'a>,
        tag_id: u8,
    },
    Neither,
}

impl<'a> ConstructorKnown<'a> {
    fn from_test_chain(
        cond_symbol: Symbol,
        cond_layout: &Layout<'a>,
        test_chain: &[(Vec<PathInstruction>, Test)],
    ) -> Self {
        match test_chain {
            [(path, test)] => match test {
                Test::IsCtor { tag_id, union, .. } if path.is_empty() => {
                    if union.alternatives.len() == 2 {
                        // excluded middle: we also know the tag_id in the fail branch
                        ConstructorKnown::Both {
                            layout: *cond_layout,
                            scrutinee: cond_symbol,
                            pass: *tag_id,
                            fail: (*tag_id == 0) as u8,
                        }
                    } else {
                        ConstructorKnown::OnlyPass {
                            layout: *cond_layout,
                            scrutinee: cond_symbol,
                            tag_id: *tag_id,
                        }
                    }
                }
                _ => ConstructorKnown::Neither,
            },
            _ => ConstructorKnown::Neither,
        }
    }
}

// TODO procs and layout are currently unused, but potentially required
// for defining optional fields?
// if not, do remove
#[allow(clippy::too_many_arguments, clippy::needless_collect)]
fn decide_to_branching<'a>(
    env: &mut Env<'a, '_>,
    procs: &mut Procs<'a>,
    layout_cache: &mut LayoutCache<'a>,
    cond_symbol: Symbol,
    cond_layout: Layout<'a>,
    ret_layout: Layout<'a>,
    decider: Decider<'a, Choice<'a>>,
    jumps: &[(u64, JoinPointId, Stmt<'a>)],
) -> Stmt<'a> {
    use Choice::*;
    use Decider::*;

    let arena = env.arena;

    match decider {
        Leaf(Jump(label)) => {
            let index = jumps
                .binary_search_by_key(&label, |ref r| r.0)
                .expect("jump not in list of jumps");

            Stmt::Jump(jumps[index].1, &[])
        }
        Leaf(Inline(expr)) => expr,
        Chain {
            test_chain,
            success,
            failure,
        } => {
            // generate a (nested) if-then-else

            let pass_expr = decide_to_branching(
                env,
                procs,
                layout_cache,
                cond_symbol,
                cond_layout,
                ret_layout,
                *success,
                jumps,
            );

            let fail_expr = decide_to_branching(
                env,
                procs,
                layout_cache,
                cond_symbol,
                cond_layout,
                ret_layout,
                *failure,
                jumps,
            );

            let chain_branch_info =
                ConstructorKnown::from_test_chain(cond_symbol, &cond_layout, &test_chain);

            let (tests, guard) = stores_and_condition(env, cond_symbol, &cond_layout, test_chain);

            let number_of_tests = tests.len() as i64 + guard.is_some() as i64;

            debug_assert!(number_of_tests > 0);

            let fail = env.arena.alloc(fail_expr);
            if number_of_tests == 1 {
                // if there is just one test, compile to a simple if-then-else

                if guard.is_none() {
                    // use knowledge about constructors for optimization
                    debug_assert_eq!(tests.len(), 1);

                    let (new_stores, lhs, rhs, _layout, _cinfo) = tests.into_iter().next().unwrap();

                    compile_test_help(
                        env,
                        chain_branch_info,
                        ret_layout,
                        new_stores,
                        lhs,
                        rhs,
                        fail,
                        pass_expr,
                    )
                } else {
                    compile_tests(env, ret_layout, tests, guard, fail, pass_expr)
                }
            } else {
                // otherwise, we use a join point so the code for the `else` case
                // is only generated once.
                let fail_jp_id = JoinPointId(env.unique_symbol());
                let jump = arena.alloc(Stmt::Jump(fail_jp_id, &[]));

                let test_stmt = compile_tests(env, ret_layout, tests, guard, jump, pass_expr);

                Stmt::Join {
                    id: fail_jp_id,
                    parameters: &[],
                    continuation: fail,
                    remainder: arena.alloc(test_stmt),
                }
            }
        }
        FanOut {
            path,
            tests,
            fallback,
        } => {
            // the cond_layout can change in the process. E.g. if the cond is a Tag, we actually
            // switch on the tag discriminant (currently an i64 value)
            // NOTE the tag discriminant is not actually loaded, `cond` can point to a tag
            let (inner_cond_symbol, cond_stores_vec, inner_cond_layout) =
                path_to_expr_help(env, cond_symbol, &path, cond_layout);

            let default_branch = decide_to_branching(
                env,
                procs,
                layout_cache,
                cond_symbol,
                cond_layout,
                ret_layout,
                *fallback,
                jumps,
            );

            let mut branches = bumpalo::collections::Vec::with_capacity_in(tests.len(), env.arena);

            let mut tag_id_sum: i64 = (0..tests.len() as i64 + 1).sum();
            let mut union_size: i64 = -1;

            for (test, decider) in tests {
                let branch = decide_to_branching(
                    env,
                    procs,
                    layout_cache,
                    cond_symbol,
                    cond_layout,
                    ret_layout,
                    decider,
                    jumps,
                );

                let tag = match test {
                    Test::IsInt(v) => v as u64,
                    Test::IsFloat(v) => v as u64,
                    Test::IsBit(v) => v as u64,
                    Test::IsByte { tag_id, .. } => tag_id as u64,
                    Test::IsCtor { tag_id, .. } => tag_id as u64,
                    other => todo!("other {:?}", other),
                };

                // branch info is only useful for refcounted values
                let branch_info = if let Test::IsCtor { tag_id, union, .. } = test {
                    tag_id_sum -= tag_id as i64;
                    union_size = union.alternatives.len() as i64;

                    BranchInfo::Constructor {
                        scrutinee: inner_cond_symbol,
                        layout: inner_cond_layout,
                        tag_id,
                    }
                } else {
                    tag_id_sum = -1;
                    BranchInfo::None
                };

                branches.push((tag, branch_info, branch));
            }

            // determine if the switch is exhaustive
            let default_branch_info = if tag_id_sum > 0 && union_size > 0 {
                BranchInfo::Constructor {
                    scrutinee: inner_cond_symbol,
                    layout: inner_cond_layout,
                    tag_id: tag_id_sum as u8,
                }
            } else {
                BranchInfo::None
            };

            // We have learned more about the exact layout of the cond (based on the path)
            // but tests are still relative to the original cond symbol
            let mut switch = if let Layout::Union(_) = inner_cond_layout {
                let tag_id_symbol = env.unique_symbol();

                let temp = Stmt::Switch {
                    cond_layout: Layout::TAG_SIZE,
                    cond_symbol: tag_id_symbol,
                    branches: branches.into_bump_slice(),
                    default_branch: (default_branch_info, env.arena.alloc(default_branch)),
                    ret_layout,
                };

                let expr = Expr::AccessAtIndex {
                    index: 0,
                    field_layouts: &[Layout::TAG_SIZE],
                    structure: inner_cond_symbol,
                    wrapped: Wrapped::MultiTagUnion,
                };

                Stmt::Let(tag_id_symbol, expr, Layout::TAG_SIZE, env.arena.alloc(temp))
            } else {
                Stmt::Switch {
                    cond_layout: inner_cond_layout,
                    cond_symbol: inner_cond_symbol,
                    branches: branches.into_bump_slice(),
                    default_branch: (default_branch_info, env.arena.alloc(default_branch)),
                    ret_layout,
                }
            };

            for (symbol, layout, expr) in cond_stores_vec.into_iter().rev() {
                switch = Stmt::Let(symbol, expr, layout, env.arena.alloc(switch));
            }

            // make a jump table based on the tests
            switch
        }
    }
}

/*
fn boolean_all<'a>(arena: &'a Bump, tests: Vec<(Expr<'a>, Expr<'a>, Layout<'a>)>) -> Expr<'a> {
    let mut expr = Expr::Bool(true);

    for (lhs, rhs, layout) in tests.into_iter().rev() {
        let test = Expr::RunLowLevel(
            LowLevel::Eq,
            bumpalo::vec![in arena; (lhs, layout.clone()), (rhs, layout.clone())].into_bump_slice(),
        );

        expr = Expr::RunLowLevel(
            LowLevel::And,
            arena.alloc([
                (test, Layout::Builtin(Builtin::Int1)),
                (expr, Layout::Builtin(Builtin::Int1)),
            ]),
        );
    }

    expr
}
*/

/// TREE TO DECIDER
///
/// Decision trees may have some redundancies, so we convert them to a Decider
/// which has special constructs to avoid code duplication when possible.

/// If a test always succeeds, we don't need to branch on it
/// this saves on work and jumps
fn test_always_succeeds(test: &Test) -> bool {
    match test {
        Test::IsCtor { union, .. } => union.alternatives.len() == 1,
        _ => false,
    }
}

fn tree_to_decider(tree: DecisionTree) -> Decider<u64> {
    use Decider::*;
    use DecisionTree::*;

    match tree {
        Match(target) => Leaf(target),

        Decision {
            path,
            mut edges,
            default,
        } => match default {
            None => match edges.len() {
                0 => panic!("compiler bug, somehow created an empty decision tree"),
                1 => {
                    let (_, sub_tree) = edges.remove(0);

                    tree_to_decider(sub_tree)
                }
                2 => {
                    let (_, failure_tree) = edges.remove(1);
                    let (test, success_tree) = edges.remove(0);

                    if test_always_succeeds(&test) {
                        tree_to_decider(success_tree)
                    } else {
                        to_chain(path, test, success_tree, failure_tree)
                    }
                }

                _ => {
                    let fallback = edges.remove(edges.len() - 1).1;

                    let necessary_tests = edges
                        .into_iter()
                        .map(|(test, decider)| (test, tree_to_decider(decider)))
                        .collect();

                    FanOut {
                        path,
                        tests: necessary_tests,
                        fallback: Box::new(tree_to_decider(fallback)),
                    }
                }
            },

            Some(last) => match edges.len() {
                0 => tree_to_decider(*last),
                1 => {
                    let failure_tree = *last;
                    let (test, success_tree) = edges.remove(0);

                    if test_always_succeeds(&test) {
                        tree_to_decider(success_tree)
                    } else {
                        to_chain(path, test, success_tree, failure_tree)
                    }
                }

                _ => {
                    let fallback = *last;

                    let necessary_tests = edges
                        .into_iter()
                        .map(|(test, decider)| (test, tree_to_decider(decider)))
                        .collect();

                    FanOut {
                        path,
                        tests: necessary_tests,
                        fallback: Box::new(tree_to_decider(fallback)),
                    }
                }
            },
        },
    }
}

fn to_chain<'a>(
    path: Vec<PathInstruction>,
    test: Test<'a>,
    success_tree: DecisionTree<'a>,
    failure_tree: DecisionTree<'a>,
) -> Decider<'a, u64> {
    use Decider::*;

    let failure = tree_to_decider(failure_tree);

    match tree_to_decider(success_tree) {
        Chain {
            mut test_chain,
            success,
            failure: sub_failure,
        } if failure == *sub_failure => {
            test_chain.push((path, test));

            Chain {
                test_chain,
                success,
                failure: Box::new(failure),
            }
        }

        success => Chain {
            test_chain: vec![(path, test)],
            success: Box::new(success),
            failure: Box::new(failure),
        },
    }
}

/// INSERT CHOICES
///
/// If a target appears exactly once in a Decider, the corresponding expression
/// can be inlined. Whether things are inlined or jumps is called a "choice".

fn count_targets(targets: &mut bumpalo::collections::Vec<u64>, initial: &Decider<u64>) {
    use Decider::*;

    let mut stack = vec![initial];

    while let Some(decision_tree) = stack.pop() {
        match decision_tree {
            Leaf(target) => {
                targets[*target as usize] += 1;
            }

            Chain {
                success, failure, ..
            } => {
                stack.push(success);
                stack.push(failure);
            }

            FanOut {
                tests, fallback, ..
            } => {
                stack.push(fallback);

                for (_, decider) in tests {
                    stack.push(decider);
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn create_choices<'a>(
    target_counts: &bumpalo::collections::Vec<'a, u64>,
    target: u64,
    branch: Stmt<'a>,
) -> ((u64, Choice<'a>), Option<(u64, Stmt<'a>)>) {
    match target_counts.get(target as usize) {
        None => unreachable!(
            "this should never happen: {:?} not in {:?}",
            target, target_counts
        ),
        Some(1) => ((target, Choice::Inline(branch)), None),
        Some(_) => ((target, Choice::Jump(target)), Some((target, branch))),
    }
}

fn insert_choices<'a>(
    choice_dict: &MutMap<u64, Choice<'a>>,
    decider: Decider<'a, u64>,
) -> Decider<'a, Choice<'a>> {
    use Decider::*;
    match decider {
        Leaf(target) => {
            // TODO remove clone
            // Only targes that appear once are Inline, so it's safe to remove them from the dict.
            Leaf(choice_dict[&target].clone())
        }

        Chain {
            test_chain,
            success,
            failure,
        } => Chain {
            test_chain,
            success: Box::new(insert_choices(choice_dict, *success)),
            failure: Box::new(insert_choices(choice_dict, *failure)),
        },

        FanOut {
            path,
            tests,
            fallback,
        } => FanOut {
            path,
            tests: tests
                .into_iter()
                .map(|(test, nested)| (test, insert_choices(choice_dict, nested)))
                .collect(),
            fallback: Box::new(insert_choices(choice_dict, *fallback)),
        },
    }
}
