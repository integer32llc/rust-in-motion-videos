# Unit 1 - Module 9: Methods

| Example | Description | Compiles? |
|---------|-------------|-----------|
| [01-00-shoot-puck-function](01-00-shoot-puck-function) | Defining a function named `shoot_puck` that takes a `HockeyPlayer` parameter | ✅ |
| [02-55-shoot-puck-method](02-55-shoot-puck-method) | Changing the `shoot_puck` function to be a method implemented on `HockeyPlayer` | ✅ |
| [05-07-associated-function](05-07-associated-function) | Defining a function named `new` associated with `HockeyPlayer` | ✅ |
| [06-13-shoot-puck-takes-ownership](06-13-shoot-puck-takes-ownership) | We can't call the `shoot_puck` method twice because it takes ownership of `self` | ❌ |
| [06-40-shoot-puck-borrows-self](06-40-shoot-puck-borrows-self) | Fixing the previous example by changing `self` to `&self` to borrow | ✅ |
| [06-59-cant-modify-immutable-self](06-59-cant-modify-immutable-self) | If we borrow self immutably, we're not allowed to modify or write to `self` | ❌ |
| [07-32-shoot-puck-borrows-self-mutably](07-32-shoot-puck-borrows-self-mutably) | Fixing the previous example by changing `&self` to `&mut self` to borrow mutably | ✅ |
