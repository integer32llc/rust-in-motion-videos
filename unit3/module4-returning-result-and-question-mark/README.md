# Unit 3 - Module 4: Returning `Result` and using Question Mark

| Example | Description | Compiles? |
|---------|-------------|-----------|
| [01-20-save-status](01-20-save-status) | Starting a `save_status` function that might fail | ❌ |
| [02-40-save-to-database-using-match](02-40-save-to-database-using-match) | Calling `save_to_database` from `save_status` and handling the `Result` using `match` | ✅ |
| [03-52-save-to-database-using-question-mark](03-52-save-to-database-using-question-mark) | Using `?` instead of the `match` in the previous example | ✅ |
| [04-44-must-return-result](04-44-must-return-result) | Question mark called on `Result` can only be used in a function that returns `Result` | ❌ |
| [05-21-fix-by-changing-return-type](05-21-fix-by-changing-return-type) | Fixing the previous example by changing the return type to `Result` | ✅ |
| [05-56-question-mark-on-option](05-56-question-mark-on-option) | Calling question mark on `Option` | ✅ |
| [06-40-q-in-main](06-40-q-in-main) | Using question mark in `main` without a return type specified won't ever work | ❌ |
| [07-08-q-in-main-by-returning-result](07-08-q-in-main-by-returning-result) | Using question mark in `main` requires changing the return type to `Result` (only works in 1.26.0+) | ❌* |
| [07-26-q-in-tests](07-26-q-in-tests) | Using question mark in `tests` requires changing the return type to `Result` (only works in 1.28.0+) | ❌* |

\* These examples don't compile in the videos where we use 1.24.1, but will start compiling in the version specified.
