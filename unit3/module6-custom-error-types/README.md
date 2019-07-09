# Unit 3 - Module 6: Custom Error Types

| Example | Description | Compiles? |
|---------|-------------|-----------|
| [00-51-box-error](00-51-box-error) | Defining a function using `Box<Error>` in the returned `Result` | ✅ |
| [02-32-using-box-error-fns](02-32-using-box-error-fns) | Using functions defined with `Box<Error>` in an application | ✅* |
| [04-00-document-storage-box-error](04-00-document-storage-box-error) | First attempt at the `create_document` function of the document storage service using `Box<Error>` | ✅ |
| [06-12-document-storage-custom-error-enum](06-12-document-storage-custom-error-enum) | Defining a custom error enum | ✅ |
| [06-44-document-storage-implementing-error](06-44-document-storage-implementing-error) | Implementing the `Error` trait on the error enum | ❌ |
| [08-05-document-storage-implementing-from](08-05-document-storage-implementing-from) | Implementing the `From` trait on the error enum | ✅ |
| [09-02-document-storage-result-alias](09-02-document-storage-result-alias) | Adding a `Result` alias for convenience | ✅ |

\* This example compiles successfully but might panic at runtime depending on the ways in which you run it.
