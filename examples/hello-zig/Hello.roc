app "hello-world"
    packages { base: "platform" }
    imports []
    provides [ main ] to base

greeting =
    hi = "Hello"
    name = "World"

    "\(hi), \(name)!"

main = greeting
