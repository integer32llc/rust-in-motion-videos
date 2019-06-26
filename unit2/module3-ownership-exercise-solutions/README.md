# Unit 2 - Module 3: Ownership Exercise Solutions

| Example | Description | Compiles? |
|---------|-------------|-----------|
| [00-48-starting-exercise](00-48-starting-exercise) | An exercise for you to get some experience with ownership, moving, and cloning. Won't compile until you fix it! | ❌ |
| [01-11-working-solution](01-11-working-solution) | One working solution to the exercise | ✅ |
| [02-06-without-cloning](02-06-without-cloning) | Without cloning, `s` is moved into `pluralize`, gets cleaned up, and `main` can't use `s` again | ❌ |
| [02-35-cloning-too-late](02-35-cloning-too-late) | Attempting to clone `s` after it has been moved into `pluralize` doesn't work either | ❌ |
| [02-53-calling-pluralize-directly](02-53-calling-pluralize-directly) | Calling `pluralize` within the `println!` works as long as the `clone` is present | ✅ |
| [03-04-calling-pluralize-directly-without-clone](03-04-calling-pluralize-directly-without-clone) | Not having `clone` in the previous solution results in different errors | ❌ |
