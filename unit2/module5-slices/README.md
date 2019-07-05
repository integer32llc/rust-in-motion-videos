# Unit 2 - Module 5: Slices

| Example | Description | Compiles? |
|---------|-------------|-----------|
| [01-12-creating-a-string-slice](01-12-creating-a-string-slice) | String slices created from a variety of ranges through 1:48 | ✅ |
| [01-50-slices-of-arrays-and-vectors](01-50-slices-of-arrays-and-vectors) | Slices of arrays and vectors | ✅ |
| [02-37-using-invalid-indices](02-37-using-invalid-indices) | Using an index past the end of a vector | ✅* |
| [03-25-using-invalid-char-boundaries](03-25-using-invalid-char-boundaries) | Using a string slice in the middle of a multi-byte Unicode character | ✅* |
| [04-08-slices-as-parameters](04-08-slices-as-parameters) | Illustrating why slices are better as parameters | ✅ |
| [04-55-string-slices-as-parameters](04-55-string-slices-as-parameters) | Illustrating why string slices are better as parameters | ✅ |
| [06-04-string-deref](06-04-string-deref) | `&String` coerces to `&str` when calling functions | ✅ |
| [06-16-array-vec-deref](06-16-array-vec-deref) | `&Vec<T>` and `[T; n]` coerce to `&[T]` when calling functions | ✅ |

\* These examples compile but panic at runtime.
