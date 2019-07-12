# Unit 4 - Module 3: Visualizing Lifetimes to Understand Borrow Checker Errors

| Example | Description | Compiles? |
|---------|-------------|-----------|
| [00-56-ex1-returning-ref-from-inner-scope](00-56-ex1-returning-ref-from-inner-scope) | Attempting to return a reference from an inner scope | ❌ |
| [02-13-fixing-ex1](02-13-fixing-ex1) | Fixing Ex #1 by moving the use of the reference into the inner scope | ✅ |
| [02-24-alt-fix-for-ex1](02-24-alt-fix-for-ex1) | Fixing Ex #1 by moving the creation of the vector out of the inner scope | ✅ |
| [02-41-ex2-returning-ref-from-fn](02-41-ex2-returning-ref-from-fn) | Attempting to return a reference from a function | ❌ |
| [03-57-fixing-ex2](03-57-fixing-ex2) | Fixing Ex #2 by returning the owned value from the function | ✅ |
| [04-17-ex3-ref-moved-value](04-17-ex3-ref-moved-value) | Attempting to reference a moved value | ❌ |
| [05-05-ex3-still-invalid](05-05-ex3-still-invalid) | Attempting to fix Ex #3 by rearranging lines still doesn't make the reference valid | ❌ |
| [05-29-fixing-ex3](05-29-fixing-ex3) | Fixing Ex #3 by ending the borrow before the move | ✅ |
| [06-05-ex4-self-referential-struct](06-05-ex4-self-referential-struct) | Attempting to store a value and a reference to that value in the same struct | ❌ |
| [07-48-ex5-refs-in-hashmap](07-48-ex5-refs-in-hashmap) | Attempting to store references in a `HashMap` | ❌ |
| [08-58-inner-scope-doesnt-fix](08-58-inner-scope-doesnt-fix) | Attempting to fix Ex #5 by adding an inner scope only makes the problem more obvious | ❌ |
| [09-17-fixing-ex5](09-17-fixing-ex5) | Fixing Ex #5 by rearranging so the `HashMap` goes out of scope first | ✅ |
| [09-32-reading-strings-in-a-loop](09-32-reading-strings-in-a-loop) | Reading the input in a loop means the references don't live long enough again | ❌ |
| [09-57-storing-owned-values-in-hashmap](09-57-storing-owned-values-in-hashmap) | Fixing the loop version by storing owned values instead of references | ✅ |
