# rb-04-3 Fixing Ownership

## Things learned

- One way to return a value from a function without
  recieving the reference, is to just send the value itself.
- Another way is to just send a string literal, this works
  only if we don't need to modify the string further, ex:

```rust
fn return_a_string() -> &'static str {
  "Hello world"
}
```

- If we are trying to get a result from an input sent to a
  function, it's best to:
  1. Not change ownership by sending the value itself, we
     should send a reference to the value.
  2. Try to clone an input an do the changes on the clone
     instead.
  3. If it's possible to not clone an entire `Vec` and first
     do a function like `join` that modifies a copy and sends it
     back, let's try to do that.
- When dereferencing and assigning a value, we try to push the
  ownership to the variable, which does not work on Strings but
  does on integers.
- Refrain from creating two mutable references to the same data
  from being used at the same time.
