# rb-09 Error handling

## Things learned

- Whenever a program crashes, Rust calles it `panic`. We
  can also make the program panic by running `panic!(message)`
  and adding a message, the program crashes and prints the
  message.
- The `Result<T, E>` type is used when an operation has a chance
  of failing, and we would like to handle the error instead
  of crashing the program. It can be used with a `match` expression.
  When handling it, the two options are `Ok` and `Err`, where Ok
  will have a value of the generic type T and Err of E.
- Standard errors have a property called `kind` that is also
  an enum that explains the error so we can handle it on a `match`.
- Reminder that a `_` catches the other values of an enum.
- Another method we can call with `Result` is `.unwrap()`, it returns
  the value if the result is Ok and it panics the program if it is
  an error. Or better yet, `.expect(message)` lets us send a
  particular message when the program panics with an Err.
- The `?` operator is used to propagate errors. Works like this: when
  the expression in case returns a `Result` with an Ok, the variable
  will be assigned that value, in case it fails it returns the value.

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

- If we also define `impl From<io::Error>` for `OurError` to construct
  an instance of `OurError` from an `io::Error`, then the `?` operator
  calls in the body of `read_username_from_file` will call from and
  convert the error types without needing to add any more code to the function.
  Better yet:

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

- The `?` operator can also be used by `Option`, nice!

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

- When a main function returns a `Result<(), E>`, the executable will
  exit with a value of 0 if main returns `Ok(())` and will exit with a
  nonzero value if main returns an `Err` value.
- "When your code performs an operation that could put a user at risk if it’s
  called using invalid values, your code should verify the values are valid
  first and panic if the values aren’t valid. This is mostly for safety
  reasons: attempting to operate on invalid data can expose your code to
  vulnerabilities."
