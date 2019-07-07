# Unit 3 - Module 2: Panicking When Something Goes Wrong

| Example | Description | Compiles? |
|---------|-------------|-----------|
| [02-08-panic-macro](02-08-panic-macro) | A plain `panic!` call | ✅* |
| [02-28-panic-with-message](02-28-panic-with-message) | Calling `panic!` with a message | ✅* |
| [02-50-message-with-interpolated-values](02-50-message-with-interpolated-values) | Calling `panic!` with a format string and interpolated values | ✅* |
| [02-58-recognized-platform](02-58-recognized-platform) | Simulating the user entering a platform we recognize and support | ✅ |
| [04-05-unknown-platform](04-05-unknown-platform) | Simulating the user entering an unknown, unsupported platform | ✅* |
| [04-37-out-of-bounds](04-37-out-of-bounds) | Out of bounds memory access | ✅* |
| [05-46-http-crate](05-46-http-crate) | Using the `http` crate in a way that it panics | ✅* |
| [06-06-fixing-http-usage](06-06-fixing-http-usage) | The only way to fix the previous example is by editing the code to have a valid static string | ✅ |
| [07-23-unreachable](07-23-unreachable) | A case that should be unreachable | ✅* |
| [08-41-unimplemented](08-41-unimplemented) | Replacing the TODO comments from the previous example with unimplemented | ✅* |

\* These examples compile but panic at runtime.
