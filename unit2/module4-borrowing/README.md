# Unit 2 - Module 4: Borrowing

| Example | Description | Compiles? |
|---------|-------------|-----------|
| [00-02-working-solution](00-02-working-solution) | The working solution to the exercise that used `clone` | ✅ |
| [01-14-borrowing-person](01-14-borrowing-person) | Borrowing a `Person` instance | ✅ |
| [02-26-more-fields-on-person](02-26-more-fields-on-person) | Cloning many fields to be able to use one would be inefficient | ✅ |
| [03-53-creating-invalid-reference-isnt-allowed](03-53-creating-invalid-reference-isnt-allowed) | The compiler prevents us from returning a reference to a value that gets cleaned up at the end of the function. | ❌ |
| [05-05-return-owned-value-instead](05-05-return-owned-value-instead) | The fix to the previous problem is to return an owned value that gets moved out of the function instead. | ✅ |
| [05-25-pluralize-with-borrowing](05-25-pluralize-with-borrowing) | Changing the `pluralize` function to borrow instead; the body of `pluralize` still needs to be fixed | ❌ |
| [06-30-call-to-owned-in-pluralize](06-30-call-to-owned-in-pluralize) | Fix the body of `pluralize` by calling `to_owned` | ✅ |
