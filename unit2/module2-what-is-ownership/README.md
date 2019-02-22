# Unit 2 - Module 2: What is ownership?

| Example | Description | Compiles? |
|---------|-------------|-----------|
| [02-19-c-explicit-memory-management](02-19-c-explicit-memory-management) | C code that manually allocates and frees memory. Compile with your favorite C compiler, not `cargo`! | ✅ |
| [05-15-creating-a-string](05-15-creating-a-string) | Creating a `String` instance from a string literal | ✅ |
| [05-41-modifying-string](05-41-modifying-string) | Adding more data to a `String`, which might allocate more space | ✅ |
| [06-21-moving-ownership](06-21-moving-ownership) | Moving ownership of a `String` into a function | ✅ |
| [06-48-cant-use-moved-value](06-48-cant-use-moved-value) | Adding a `println!` to the previous example to show we can't use a value after it's been moved | ❌ |
| [07-06-returning-ownership](07-06-returning-ownership) | Returning ownership of a value out of a function | ✅ |
| [07-50-clone](07-50-clone) | Cloning a `String` and transferring ownership of the clone | ✅ |
| [08-30-ownership-exercise](08-30-ownership-exercise) | An exercise for you to get some experience with ownership, moving, and cloning. Won't compile until you fix it! | ❌ |
