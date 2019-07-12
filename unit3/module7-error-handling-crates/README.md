# Unit 3 - Module 7: Error Handling Crates

| Example | Description | Compiles? |
|---------|-------------|-----------|
| [00-28-document-storage-m6](00-28-document-storage-m6) | Review of the document storage service example from Module 6, plus more code for better illustration in this module | ✅* |
| [01-49-quick-error](01-49-quick-error) | Using quick-error to improve the document storage service | ✅* |
| [03-31-quick-error-with-context](03-31-quick-error-with-context) | Changing the quick-error example to use the `context` features | ✅* |
| [04-17-error-chain](04-17-error-chain) | Using error-chain to improve the document storage service | ✅* |
| [07-55-failure](07-55-failure) | Using failure to improve the document storage service | ✅* |

\* These examples compile and run successfully the first time, but will panic if you don't delete the created files before running a second time.

## Backtrace crate compatibility notes

If you are using Rust 1.24.1 as we are in the videos, changes in newer versions of the `backtrace` crate won't compile. This affects the `error-chain` and `failure` examples. To fix them, remove any `Cargo.lock` and add this to the `[dependencies]` in `Cargo.toml`:

```toml
backtrace = "=0.3.18"
```
