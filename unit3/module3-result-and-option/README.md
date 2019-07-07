# Unit 3 - Module 3: Handling `Result` and `Option`

| Example | Description | Compiles? |
|---------|-------------|-----------|
| [01-37-serde-json](01-37-serde-json) | Parsing valid and invalid JSON to see the `Result` | ✅ |
| [02-49-lists](02-49-lists) | Getting the last item from nonempty and empty lists to see the `Option` | ✅ |
| [03-48-using-serde-json-result](03-48-using-serde-json-result) | Trying to use a `Result` as if it were the success value isn't allowed | ❌ |
| [04-03-extract-value-with-match](04-03-extract-value-with-match) | Using a `match` expression to extract the inner value from the `Ok` | ✅ |
| [04-35-default-value-if-failure](04-35-default-value-if-failure) | Handling parsing failure by returning a default value. | ✅ |
| [05-01-handling-different-errors](05-01-handling-different-errors) | Taking different actions based on the properties of the error | ❌ |
| [05-50-recoverable-to-unrecoverable](05-50-recoverable-to-unrecoverable) | Turning a recoverable `Err` into an unrecoverable `panic` | ✅* |

\* These examples compile but panic at runtime.
