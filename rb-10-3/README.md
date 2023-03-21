# rb-10-3 Lifetimes

## Things learned

- Not a concept in most common programming languages, but
  it's about anotating a reference so the compiler knows the
  lifetime it will have on our program.
- _"The main aim of lifetimes is to prevent dangling references."_
- Even though Rust allows you to declare a variable without a value
  it won't let you compile the program unless it sees you assigned
  a value before using it at some point in your program.
- I need to read more about this here -> https://rust-book.cs.brown.edu/ch10-03-lifetime-syntax.html
- Ok so, how it goes is, if you anotate a function, the returned reference
  will last as long as the scope of the function that called it is valid.
- You can just specify the lifetime of one the parameters. Example:

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

- A "lifetime ellision rule" is a deterministic path that is
  encoded into the rust compiler.
- This is the kind of thing I need to just read back whenever I
  encounter an issue with it, to focus so much on it right now will
  just hinder my progress just because I didn't understand one thing.
