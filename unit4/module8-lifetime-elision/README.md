# Unit 4 - Module 8: Lifetime Elision

| Example | Description | Compiles? |
|---------|-------------|-----------|
| [01-42-first-rule-applies](01-42-first-rule-applies) | Rule #1: Each reference in the parameters gets its own lifetime | ❌ |
| [02-01-second-rule-applies](02-01-second-rule-applies) | Rule #2: If there is 1 lifetime in the parameters, any returned reference gets that lifetime | ✅ |
| [02-34-third-rule-applies](02-34-third-rule-applies) | Rule #3: If there is a `self` parameter, returned references get the lifetime of `self` | ✅ |
| [03-43-fn-returning-reference](03-43-fn-returning-reference) | Returned reference gets the same lifetime as the one reference parameter | ✅ |
| [04-16-stemmer-example-without-lifetime-params](04-16-stemmer-example-without-lifetime-params) | The `Stemmer` example from modules 4 and 6 without any lifetime parameters | ❌ |
| [05-05-stemmer-fix-partially-elided](05-05-stemmer-fix-partially-elided) | The fix for the `Stemmer` example is to specify lifetime parameters for some of the references to communicate the correct relationship | ✅ |
