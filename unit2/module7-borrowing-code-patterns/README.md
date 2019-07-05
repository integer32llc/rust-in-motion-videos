# Unit 2 - Module 7: Borrowing Code Patterns

| Example | Description | Compiles? |
|---------|-------------|-----------|
| [00-59-multiple-immutable-references](00-59-multiple-immutable-references) | Multiple immutable references in the same scope is allowed | ✅ |
| [01-23-immutable-and-mutable](01-23-immutable-and-mutable) | Multiple immutable references and one mutabel reference in the same scope isn't allowed | ❌ |
| [01-43-mutable-ending-before-immutable](01-43-mutable-ending-before-immutable) | A mutable reference that ends before taking immutable references is allowed | ✅ |
| [02-16-mutable-after-immutable](02-16-mutable-after-immutable) | A mutable reference taken after taking immutable references | ❌* |
| [03-28-new-scopes](03-28-new-scopes) | Fix the previous example by adding a new scope to end borrows early | ✅ |
| [04-24-borrows-in-one-statement](04-24-borrows-in-one-statement) | An immutable borrow and a mutable borrow within one statement that conflict with each other | ❌* |
| [05-24-temporary-variable](05-24-temporary-variable) | Fix the previous example by adding a temporary variable to end the immutable borrow before the mutable borrow | ✅ |
| [06-26-counting-word-frequencies](06-26-counting-word-frequencies) | Count how many times each word occurs in some text | ❌*† |
| [07-47-entry](07-47-entry) | Use `entry` to handle occupied vs vacant | ✅ |
| [08-07-monster-final-breath](08-07-monster-final-breath) | A `Monster` struct with a `final_breath` method that uses fields in one method | ✅ |
| [08-49-monster-heal](08-49-monster-heal) | Extracting a `heal` method that borrows all of `self` | ❌ |
| [09-37-split-up-structs](09-37-split-up-structs) | Fix the previous example by extracting a `Stats` struct from `Monster` | ✅ |

\* These examples don't compile in the videos where we use 1.24.1, but they will start compiling with the Non-Lexical Lifetimes feature, released with 2018 Edition Rust in 1.31 and 2015 Edition Rust in 1.36.

† While this example will compile with Non-Lexical Lifetimes, as written it will do an extra hash+lookup, so using `entry` is preferred.
