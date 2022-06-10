// ⚠️ GENERATED CODE ⚠️ - this entire file was generated by the `roc-bindgen` CLI

#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::undocumented_unsafe_blocks)]

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Rbt {
    pub default: Job,
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy, Eq, Ord, Hash, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum discriminant_Job {
    Foo = 0,
    Job = 1,
}

impl core::fmt::Debug for discriminant_Job {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Foo => f.write_str("discriminant_Job::Foo"),
            Self::Job => f.write_str("discriminant_Job::Job"),
        }
    }
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]

pub struct Job {
    pointer: *mut union_Job,
}


#[cfg(any(
    target_arch = "arm",
    target_arch = "wasm32",
    target_arch = "x86"
))]
#[repr(C)]
union union_Job {
    Foo: core::mem::ManuallyDrop<Job_Foo>,
    Job: core::mem::ManuallyDrop<Job_Job>,
    _sizer: [u8; 4],
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
#[repr(transparent)]
struct Job_Job {
    pub f0: R1,
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
#[repr(C)]
pub struct R1 {
    pub command: Command,
    pub inputFiles: roc_std::RocList<roc_std::RocStr>,
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy, Eq, Ord, Hash, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum discriminant_Command {
    Command = 0,
}

impl core::fmt::Debug for discriminant_Command {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Command => f.write_str("discriminant_Command::Command"),
        }
    }
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[repr(C)]
pub union Command {
    Command: core::mem::ManuallyDrop<R2>,
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct R2 {
    pub tool: Tool,
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy, Eq, Ord, Hash, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum discriminant_Tool {
    FromJob = 0,
    SystemTool = 1,
}

impl core::fmt::Debug for discriminant_Tool {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::FromJob => f.write_str("discriminant_Tool::FromJob"),
            Self::SystemTool => f.write_str("discriminant_Tool::SystemTool"),
        }
    }
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "wasm32",
    target_arch = "x86"
))]
#[repr(C)]
pub union Tool {
    FromJob: core::mem::ManuallyDrop<R3>,
    SystemTool: core::mem::ManuallyDrop<R4>,
    _sizer: [u8; 20],
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[derive(Clone, Debug, Default, Eq, Ord, Hash, PartialEq, PartialOrd)]
#[repr(C)]
pub struct R4 {
    pub name: roc_std::RocStr,
    pub num: u32,
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
#[repr(C)]
pub struct R3 {
    pub job: Job,
    pub num: u32,
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[derive(Clone, Debug, Default, Eq, Ord, Hash, PartialEq, PartialOrd)]
#[repr(transparent)]
struct Job_Foo {
    pub f0: roc_std::RocStr,
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86_64"
))]
#[repr(C)]
union union_Job {
    Foo: core::mem::ManuallyDrop<Job_Foo>,
    Job: core::mem::ManuallyDrop<Job_Job>,
    _sizer: [u8; 8],
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86_64"
))]
#[repr(C)]
pub union Tool {
    FromJob: core::mem::ManuallyDrop<R3>,
    SystemTool: core::mem::ManuallyDrop<R4>,
    _sizer: [u8; 40],
}

impl Job {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    #[inline(always)]
    fn storage(&self) -> Option<&core::cell::Cell<roc_std::Storage>> {
        if self.pointer.is_null() {
            None
        } else {
            unsafe {
                Some(&*self.pointer.cast::<core::cell::Cell<roc_std::Storage>>().sub(1))
            }
        }
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "wasm32",
        target_arch = "x86"
    ))]
    /// Returns which variant this tag union holds. Note that this never includes a payload!
    pub fn discriminant(&self) -> discriminant_Job {
        // The discriminant is stored in the unused bytes at the end of the recursive pointer
        unsafe { core::mem::transmute::<u8, discriminant_Job>((self.pointer as u8) & 0b11) }
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "wasm32",
        target_arch = "x86"
    ))]
    /// Internal helper
    fn tag_discriminant(pointer: *mut union_Job, discriminant: discriminant_Job) -> *mut union_Job {
        // The discriminant is stored in the unused bytes at the end of the union pointer
        let untagged = (pointer as usize) & (!0b11 as usize);
        let tagged = untagged | (discriminant as usize);

        tagged as *mut union_Job
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "wasm32",
        target_arch = "x86"
    ))]
    /// Internal helper
    fn union_pointer(&self) -> *mut union_Job {
        // The discriminant is stored in the unused bytes at the end of the union pointer
        ((self.pointer as usize) & (!0b11 as usize)) as *mut union_Job
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Construct a tag named Foo, with the appropriate payload
    pub fn Foo(arg0: roc_std::RocStr) -> Self {
        let size = core::mem::size_of::<union_Job>();
        let align = core::mem::align_of::<union_Job>() as u32;

        unsafe {
            let ptr = crate::roc_alloc(size, align) as *mut union_Job;

            *ptr = union_Job {
                Foo: core::mem::ManuallyDrop::new(Job_Foo {
                    f0: arg0,
                })
            };

            Self {
                pointer: Self::tag_discriminant(ptr, discriminant_Job::Foo),
            }
        }
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "wasm32",
        target_arch = "x86"
    ))]
    /// Unsafely assume the given Job has a .discriminant() of Foo and convert it to Foo's payload.
    /// (Always examine .discriminant() first to make sure this is the correct variant!)
    /// Panics in debug builds if the .discriminant() doesn't return Foo.
    pub unsafe fn into_Foo(mut self) -> roc_std::RocStr {
        debug_assert_eq!(self.discriminant(), discriminant_Job::Foo);

        let payload = {
            let ptr = (self.pointer as usize & !0b11) as *mut union_Job;

            core::mem::ManuallyDrop::take(&mut (*ptr).Foo)
        };

        
        payload.f0
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "wasm32",
        target_arch = "x86"
    ))]
    /// Unsafely assume the given Job has a .discriminant() of Foo and return its payload.
    /// (Always examine .discriminant() first to make sure this is the correct variant!)
    /// Panics in debug builds if the .discriminant() doesn't return Foo.
    pub unsafe fn as_Foo(&self) -> &roc_std::RocStr {
        debug_assert_eq!(self.discriminant(), discriminant_Job::Foo);

        let payload = {
            let ptr = (self.pointer as usize & !0b11) as *mut union_Job;

            &(*ptr).Foo
        };

        
        &payload.f0
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Construct a tag named Job, with the appropriate payload
    pub fn Job(arg0: R1) -> Self {
        let size = core::mem::size_of::<union_Job>();
        let align = core::mem::align_of::<union_Job>() as u32;

        unsafe {
            let ptr = crate::roc_alloc(size, align) as *mut union_Job;

            *ptr = union_Job {
                Job: core::mem::ManuallyDrop::new(Job_Job {
                    f0: arg0,
                })
            };

            Self {
                pointer: Self::tag_discriminant(ptr, discriminant_Job::Job),
            }
        }
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "wasm32",
        target_arch = "x86"
    ))]
    /// Unsafely assume the given Job has a .discriminant() of Job and convert it to Job's payload.
    /// (Always examine .discriminant() first to make sure this is the correct variant!)
    /// Panics in debug builds if the .discriminant() doesn't return Job.
    pub unsafe fn into_Job(mut self) -> R1 {
        debug_assert_eq!(self.discriminant(), discriminant_Job::Job);

        let payload = {
            let ptr = (self.pointer as usize & !0b11) as *mut union_Job;

            core::mem::ManuallyDrop::take(&mut (*ptr).Job)
        };

        
        payload.f0
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "wasm32",
        target_arch = "x86"
    ))]
    /// Unsafely assume the given Job has a .discriminant() of Job and return its payload.
    /// (Always examine .discriminant() first to make sure this is the correct variant!)
    /// Panics in debug builds if the .discriminant() doesn't return Job.
    pub unsafe fn as_Job(&self) -> &R1 {
        debug_assert_eq!(self.discriminant(), discriminant_Job::Job);

        let payload = {
            let ptr = (self.pointer as usize & !0b11) as *mut union_Job;

            &(*ptr).Job
        };

        
        &payload.f0
    }

    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "x86_64"
    ))]
    /// Returns which variant this tag union holds. Note that this never includes a payload!
    pub fn discriminant(&self) -> discriminant_Job {
        // The discriminant is stored in the unused bytes at the end of the recursive pointer
        unsafe { core::mem::transmute::<u8, discriminant_Job>((self.pointer as u8) & 0b111) }
    }

    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "x86_64"
    ))]
    /// Internal helper
    fn tag_discriminant(pointer: *mut union_Job, discriminant: discriminant_Job) -> *mut union_Job {
        // The discriminant is stored in the unused bytes at the end of the union pointer
        let untagged = (pointer as usize) & (!0b111 as usize);
        let tagged = untagged | (discriminant as usize);

        tagged as *mut union_Job
    }

    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "x86_64"
    ))]
    /// Internal helper
    fn union_pointer(&self) -> *mut union_Job {
        // The discriminant is stored in the unused bytes at the end of the union pointer
        ((self.pointer as usize) & (!0b111 as usize)) as *mut union_Job
    }

    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume the given Job has a .discriminant() of Foo and convert it to Foo's payload.
    /// (Always examine .discriminant() first to make sure this is the correct variant!)
    /// Panics in debug builds if the .discriminant() doesn't return Foo.
    pub unsafe fn into_Foo(mut self) -> roc_std::RocStr {
        debug_assert_eq!(self.discriminant(), discriminant_Job::Foo);

        let payload = {
            let ptr = (self.pointer as usize & !0b111) as *mut union_Job;

            core::mem::ManuallyDrop::take(&mut (*ptr).Foo)
        };

        
        payload.f0
    }

    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume the given Job has a .discriminant() of Foo and return its payload.
    /// (Always examine .discriminant() first to make sure this is the correct variant!)
    /// Panics in debug builds if the .discriminant() doesn't return Foo.
    pub unsafe fn as_Foo(&self) -> &roc_std::RocStr {
        debug_assert_eq!(self.discriminant(), discriminant_Job::Foo);

        let payload = {
            let ptr = (self.pointer as usize & !0b111) as *mut union_Job;

            &(*ptr).Foo
        };

        
        &payload.f0
    }

    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume the given Job has a .discriminant() of Job and convert it to Job's payload.
    /// (Always examine .discriminant() first to make sure this is the correct variant!)
    /// Panics in debug builds if the .discriminant() doesn't return Job.
    pub unsafe fn into_Job(mut self) -> R1 {
        debug_assert_eq!(self.discriminant(), discriminant_Job::Job);

        let payload = {
            let ptr = (self.pointer as usize & !0b111) as *mut union_Job;

            core::mem::ManuallyDrop::take(&mut (*ptr).Job)
        };

        
        payload.f0
    }

    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume the given Job has a .discriminant() of Job and return its payload.
    /// (Always examine .discriminant() first to make sure this is the correct variant!)
    /// Panics in debug builds if the .discriminant() doesn't return Job.
    pub unsafe fn as_Job(&self) -> &R1 {
        debug_assert_eq!(self.discriminant(), discriminant_Job::Job);

        let payload = {
            let ptr = (self.pointer as usize & !0b111) as *mut union_Job;

            &(*ptr).Job
        };

        
        &payload.f0
    }
}

impl Drop for Job {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn drop(&mut self) {
        // We only need to do any work if there's actually a heap-allocated payload.
        if let Some(storage) = self.storage() {
            let mut new_storage = storage.get();

            // Decrement the refcount
            let needs_dealloc = !new_storage.is_readonly() && new_storage.decrease();

            if needs_dealloc {
                // Drop the payload first.
                            match self.discriminant() {
                discriminant_Job::Foo => unsafe { core::mem::ManuallyDrop::drop(&mut (&mut *self.union_pointer()).Foo) },
                discriminant_Job::Job => unsafe { core::mem::ManuallyDrop::drop(&mut (&mut *self.union_pointer()).Job) },
            }


                // Dealloc the pointer
                let alignment = core::mem::align_of::<Self>().max(core::mem::align_of::<roc_std::Storage>());

                unsafe { crate::roc_dealloc(storage.as_ptr().cast(), alignment as u32); }
            } else {
                // Write the storage back.
                storage.set(new_storage);
            }
        }
    }
}

impl Eq for Job {}

impl PartialEq for Job {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn eq(&self, other: &Self) -> bool {
            if self.discriminant() != other.discriminant() {
                return false;
            }

            unsafe {
            match self.discriminant() {
                discriminant_Job::Foo => (&*self.union_pointer()).Foo == (&*other.union_pointer()).Foo,
                discriminant_Job::Job => (&*self.union_pointer()).Job == (&*other.union_pointer()).Job,
            }
        }
    }
}

impl PartialOrd for Job {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match self.discriminant().partial_cmp(&other.discriminant()) {
            Some(core::cmp::Ordering::Equal) => {}
            not_eq => return not_eq,
        }

        unsafe {
            match self.discriminant() {
                discriminant_Job::Foo => (&*self.union_pointer()).Foo.partial_cmp(&(&*other.union_pointer()).Foo),
                discriminant_Job::Job => (&*self.union_pointer()).Job.partial_cmp(&(&*other.union_pointer()).Job),
            }
        }
    }
}

impl Ord for Job {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
            match self.discriminant().cmp(&other.discriminant()) {
                core::cmp::Ordering::Equal => {}
                not_eq => return not_eq,
            }

            unsafe {
            match self.discriminant() {
                discriminant_Job::Foo => (&*self.union_pointer()).Foo.cmp(&(&*other.union_pointer()).Foo),
                discriminant_Job::Job => (&*self.union_pointer()).Job.cmp(&(&*other.union_pointer()).Job),
            }
        }
    }
}

impl Clone for Job {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn clone(&self) -> Self {
    if let Some(storage) = self.storage() {
        let mut new_storage = storage.get();
        if !new_storage.is_readonly() {
            new_storage.increment_reference_count();
            storage.set(new_storage);
        }
    }

    Self {
        pointer: self.pointer
    }
}
}

impl core::hash::Hash for Job {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {        match self.discriminant() {
            discriminant_Job::Foo => unsafe {
                    discriminant_Job::Foo.hash(state);
                    (&*self.union_pointer()).Foo.hash(state);
                },
            discriminant_Job::Job => unsafe {
                    discriminant_Job::Job.hash(state);
                    (&*self.union_pointer()).Job.hash(state);
                },
        }
    }
}

impl core::fmt::Debug for Job {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Job::")?;

        unsafe {
            match self.discriminant() {
                discriminant_Job::Foo => f.debug_tuple("Foo")
        .field(&(&*(&*self.union_pointer()).Foo).f0)
        .finish(),
                discriminant_Job::Job => f.debug_tuple("Job")
        .field(&(&*(&*self.union_pointer()).Job).f0)
        .finish(),
            }
        }
    }
}

impl Command {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "wasm32",
        target_arch = "x86"
    ))]
    /// Returns which variant this tag union holds. Note that this never includes a payload!
    pub fn discriminant(&self) -> discriminant_Command {
        unsafe {
            let bytes = core::mem::transmute::<&Self, &[u8; core::mem::size_of::<Self>()]>(self);

            core::mem::transmute::<u8, discriminant_Command>(*bytes.as_ptr().add(16))
        }
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "wasm32",
        target_arch = "x86"
    ))]
    /// Internal helper
    fn set_discriminant(&mut self, discriminant: discriminant_Command) {
        let discriminant_ptr: *mut discriminant_Command = (self as *mut Command).cast();

        unsafe {
            *(discriminant_ptr.add(16)) = discriminant;
        }
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Construct a tag named Command, with the appropriate payload
    pub fn Command(arg0: Tool) -> Self {
        let mut answer = Self {
            Command: core::mem::ManuallyDrop::new(R2 {
                    tool: arg0,
                })
        };

        answer.set_discriminant(discriminant_Command::Command);

        answer
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume the given Command has a .discriminant() of Command and convert it to Command's payload.
    /// (Always examine .discriminant() first to make sure this is the correct variant!)
    /// Panics in debug builds if the .discriminant() doesn't return Command.
    pub unsafe fn into_Command(mut self) -> Tool {
        debug_assert_eq!(self.discriminant(), discriminant_Command::Command);

        let payload = core::mem::ManuallyDrop::take(&mut self.Command);

        
        payload.tool
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume the given Command has a .discriminant() of Command and return its payload.
    /// (Always examine .discriminant() first to make sure this is the correct variant!)
    /// Panics in debug builds if the .discriminant() doesn't return Command.
    pub unsafe fn as_Command(&self) -> &Tool {
        debug_assert_eq!(self.discriminant(), discriminant_Command::Command);

        let payload = &self.Command;

        
        &payload.tool
    }

    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "x86_64"
    ))]
    /// Returns which variant this tag union holds. Note that this never includes a payload!
    pub fn discriminant(&self) -> discriminant_Command {
        unsafe {
            let bytes = core::mem::transmute::<&Self, &[u8; core::mem::size_of::<Self>()]>(self);

            core::mem::transmute::<u8, discriminant_Command>(*bytes.as_ptr().add(32))
        }
    }

    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "x86_64"
    ))]
    /// Internal helper
    fn set_discriminant(&mut self, discriminant: discriminant_Command) {
        let discriminant_ptr: *mut discriminant_Command = (self as *mut Command).cast();

        unsafe {
            *(discriminant_ptr.add(32)) = discriminant;
        }
    }
}

impl Drop for Command {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn drop(&mut self) {
        // Drop the payloads
                    match self.discriminant() {
                discriminant_Command::Command => unsafe { core::mem::ManuallyDrop::drop(&mut self.Command) },
            }

    }
}

impl Eq for Command {}

impl PartialEq for Command {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn eq(&self, other: &Self) -> bool {
            if self.discriminant() != other.discriminant() {
                return false;
            }

            unsafe {
            match self.discriminant() {
                discriminant_Command::Command => self.Command == other.Command,
            }
        }
    }
}

impl PartialOrd for Command {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match self.discriminant().partial_cmp(&other.discriminant()) {
            Some(core::cmp::Ordering::Equal) => {}
            not_eq => return not_eq,
        }

        unsafe {
            match self.discriminant() {
                discriminant_Command::Command => self.Command.partial_cmp(&other.Command),
            }
        }
    }
}

impl Ord for Command {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
            match self.discriminant().cmp(&other.discriminant()) {
                core::cmp::Ordering::Equal => {}
                not_eq => return not_eq,
            }

            unsafe {
            match self.discriminant() {
                discriminant_Command::Command => self.Command.cmp(&other.Command),
            }
        }
    }
}

impl Clone for Command {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn clone(&self) -> Self {
        let mut answer = unsafe {
            match self.discriminant() {
                discriminant_Command::Command => Self {
                    Command: self.Command.clone(),
                },
            }

        };

        answer.set_discriminant(self.discriminant());

        answer
    }
}

impl core::hash::Hash for Command {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {        match self.discriminant() {
            discriminant_Command::Command => unsafe {
                    discriminant_Command::Command.hash(state);
                    self.Command.hash(state);
                },
        }
    }
}

impl core::fmt::Debug for Command {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Command::")?;

        unsafe {
            match self.discriminant() {
                discriminant_Command::Command => f.debug_tuple("Command")
        .field(&*self.Command)
        .finish(),
            }
        }
    }
}

impl Tool {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "wasm32",
        target_arch = "x86"
    ))]
    /// Returns which variant this tag union holds. Note that this never includes a payload!
    pub fn discriminant(&self) -> discriminant_Tool {
        unsafe {
            let bytes = core::mem::transmute::<&Self, &[u8; core::mem::size_of::<Self>()]>(self);

            core::mem::transmute::<u8, discriminant_Tool>(*bytes.as_ptr().add(16))
        }
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "wasm32",
        target_arch = "x86"
    ))]
    /// Internal helper
    fn set_discriminant(&mut self, discriminant: discriminant_Tool) {
        let discriminant_ptr: *mut discriminant_Tool = (self as *mut Tool).cast();

        unsafe {
            *(discriminant_ptr.add(16)) = discriminant;
        }
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Construct a tag named FromJob, with the appropriate payload
    pub fn FromJob(arg0: Job, arg1: u32) -> Self {
        let mut answer = Self {
            FromJob: core::mem::ManuallyDrop::new(R3 {
                    job: arg0,
                    num: arg1,
                })
        };

        answer.set_discriminant(discriminant_Tool::FromJob);

        answer
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume the given Tool has a .discriminant() of FromJob and convert it to FromJob's payload.
    /// (Always examine .discriminant() first to make sure this is the correct variant!)
    /// Panics in debug builds if the .discriminant() doesn't return FromJob.
    pub unsafe fn into_FromJob(mut self) -> (Job, u32) {
        debug_assert_eq!(self.discriminant(), discriminant_Tool::FromJob);

        let payload = core::mem::ManuallyDrop::take(&mut self.FromJob);

        (
            payload.job, 
            payload.num
        )
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume the given Tool has a .discriminant() of FromJob and return its payload.
    /// (Always examine .discriminant() first to make sure this is the correct variant!)
    /// Panics in debug builds if the .discriminant() doesn't return FromJob.
    pub unsafe fn as_FromJob(&self) -> (&Job, &u32) {
        debug_assert_eq!(self.discriminant(), discriminant_Tool::FromJob);

        let payload = &self.FromJob;

        (
            &payload.job, 
            &payload.num
        )
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Construct a tag named SystemTool, with the appropriate payload
    pub fn SystemTool(arg0: roc_std::RocStr, arg1: u32) -> Self {
        let mut answer = Self {
            SystemTool: core::mem::ManuallyDrop::new(R4 {
                    name: arg0,
                    num: arg1,
                })
        };

        answer.set_discriminant(discriminant_Tool::SystemTool);

        answer
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume the given Tool has a .discriminant() of SystemTool and convert it to SystemTool's payload.
    /// (Always examine .discriminant() first to make sure this is the correct variant!)
    /// Panics in debug builds if the .discriminant() doesn't return SystemTool.
    pub unsafe fn into_SystemTool(mut self) -> (roc_std::RocStr, u32) {
        debug_assert_eq!(self.discriminant(), discriminant_Tool::SystemTool);

        let payload = core::mem::ManuallyDrop::take(&mut self.SystemTool);

        (
            payload.name, 
            payload.num
        )
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume the given Tool has a .discriminant() of SystemTool and return its payload.
    /// (Always examine .discriminant() first to make sure this is the correct variant!)
    /// Panics in debug builds if the .discriminant() doesn't return SystemTool.
    pub unsafe fn as_SystemTool(&self) -> (&roc_std::RocStr, &u32) {
        debug_assert_eq!(self.discriminant(), discriminant_Tool::SystemTool);

        let payload = &self.SystemTool;

        (
            &payload.name, 
            &payload.num
        )
    }

    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "x86_64"
    ))]
    /// Returns which variant this tag union holds. Note that this never includes a payload!
    pub fn discriminant(&self) -> discriminant_Tool {
        unsafe {
            let bytes = core::mem::transmute::<&Self, &[u8; core::mem::size_of::<Self>()]>(self);

            core::mem::transmute::<u8, discriminant_Tool>(*bytes.as_ptr().add(32))
        }
    }

    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "x86_64"
    ))]
    /// Internal helper
    fn set_discriminant(&mut self, discriminant: discriminant_Tool) {
        let discriminant_ptr: *mut discriminant_Tool = (self as *mut Tool).cast();

        unsafe {
            *(discriminant_ptr.add(32)) = discriminant;
        }
    }
}

impl Drop for Tool {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn drop(&mut self) {
        // Drop the payloads
                    match self.discriminant() {
                discriminant_Tool::FromJob => unsafe { core::mem::ManuallyDrop::drop(&mut self.FromJob) },
                discriminant_Tool::SystemTool => unsafe { core::mem::ManuallyDrop::drop(&mut self.SystemTool) },
            }

    }
}

impl Eq for Tool {}

impl PartialEq for Tool {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn eq(&self, other: &Self) -> bool {
            if self.discriminant() != other.discriminant() {
                return false;
            }

            unsafe {
            match self.discriminant() {
                discriminant_Tool::FromJob => self.FromJob == other.FromJob,
                discriminant_Tool::SystemTool => self.SystemTool == other.SystemTool,
            }
        }
    }
}

impl PartialOrd for Tool {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match self.discriminant().partial_cmp(&other.discriminant()) {
            Some(core::cmp::Ordering::Equal) => {}
            not_eq => return not_eq,
        }

        unsafe {
            match self.discriminant() {
                discriminant_Tool::FromJob => self.FromJob.partial_cmp(&other.FromJob),
                discriminant_Tool::SystemTool => self.SystemTool.partial_cmp(&other.SystemTool),
            }
        }
    }
}

impl Ord for Tool {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
            match self.discriminant().cmp(&other.discriminant()) {
                core::cmp::Ordering::Equal => {}
                not_eq => return not_eq,
            }

            unsafe {
            match self.discriminant() {
                discriminant_Tool::FromJob => self.FromJob.cmp(&other.FromJob),
                discriminant_Tool::SystemTool => self.SystemTool.cmp(&other.SystemTool),
            }
        }
    }
}

impl Clone for Tool {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn clone(&self) -> Self {
        let mut answer = unsafe {
            match self.discriminant() {
                discriminant_Tool::FromJob => Self {
                    FromJob: self.FromJob.clone(),
                },
                discriminant_Tool::SystemTool => Self {
                    SystemTool: self.SystemTool.clone(),
                },
            }

        };

        answer.set_discriminant(self.discriminant());

        answer
    }
}

impl core::hash::Hash for Tool {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {        match self.discriminant() {
            discriminant_Tool::FromJob => unsafe {
                    discriminant_Tool::FromJob.hash(state);
                    self.FromJob.hash(state);
                },
            discriminant_Tool::SystemTool => unsafe {
                    discriminant_Tool::SystemTool.hash(state);
                    self.SystemTool.hash(state);
                },
        }
    }
}

impl core::fmt::Debug for Tool {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Tool::")?;

        unsafe {
            match self.discriminant() {
                discriminant_Tool::FromJob => f.debug_tuple("FromJob")
        .field(&*self.FromJob)
        .finish(),
                discriminant_Tool::SystemTool => f.debug_tuple("SystemTool")
        .field(&*self.SystemTool)
        .finish(),
            }
        }
    }
}