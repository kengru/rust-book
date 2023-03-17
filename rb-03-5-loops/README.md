# rb-03-5 Loops

## Things learned

- Loops are expressions, which means they can return a value, something
  like this is possible:

```rust
let result = loop {
  break 2;
}
```

- Loops can have labels!! Labels can be used to `break` or `continue` an specific loop.
- `while` loops work just normal.
- `for` loops can be used to loop through a collection such as an array using the
  `in` keyword as in: `for element in array {}`
- Ranges are like this: `1..4` and can be used in a for to iterate a set number of times.
