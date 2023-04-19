# rb-18-1 Patterns Places

## Things learned

- The particular pattern _ will match anything, but it never
  binds to a variable, so it’s often used in the last match arm.
  The _ pattern can be useful when you want to ignore any value
  not specified, for example.
- _Shadowed variables_ is a term for variables that are created
  when trying to match a pattern either in a `match` or an `if let`.
- Shadowed variables are only available inside the scope those
  patterns produce.
- `while let` is useful for when we are tying to pop all the
  elements of a vector for example. Because it runs until a condition
  is no longer true such as checking for `Some()` value until you
  recieve a `None`.
- The `let` keyword is actually the pattern maker, which means than
  when we create a variable it's actually functioning with the rules
  explained before. I just need to think of it like the let is creating
  a _shadowed variable_ that works in the scope of the entire program.
- Destructuring when declaring a variable works just fine, it just
  needs to be a tuple: `let (x, y) = (1, 2);`
- Function parameters, `let` statements, `for` loops only accept
  _irrefutable_ patterns, which means that they need to match just one
  thing, and it's imposible to get anything else.
