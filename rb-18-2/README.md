# rb-18-2 Pattern Syntax

## Things learned

- `match` can basically act as a switch statements from any other
  language, since match can check for literal values such as any number
  from a single variable.
- When trying to use another variable on a match statement, the arm that
  tries to do it actually creates a new shadowed variable instead of matching
  with an outer variable.
- `|` is used for an `or` operator when matching a value.
- `..=` is to match on a inclusive range of values. It can also be used with
  characters.

```rust
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}
```

- You can nest the destructuring when matching an enum. Which means you can
  match the enums inside an enum.
- Something I knew but variables that start with a `_` don't give an unused
  variable warning.
- With values that have many parts, we can use the .. syntax to use specific
  parts and ignore the rest, avoiding the need to list underscores for each
  ignored value.
- A match guard is an additional if condition, specified after the pattern in
  a match arm, that must also match for that arm to be chosen. Match guards are
  useful for expressing more complex ideas than a pattern alone allows. When
  match guard expressions are on a match pattern, the Rust compiler will not
  try to check for exhaustiveness.
- The at operator @ lets us create a variable that holds a value at the same
  time as we’re testing that value for a pattern match.
