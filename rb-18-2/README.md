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
