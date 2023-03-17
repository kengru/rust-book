# rb-03-1 Variables

## Things learned

- Constants are always inmutable and cannot be made mutable, and must be
  annotaded. Example: `const NUMBER_FIFTY: u32 = 50;`
- Variables must have the `mut` keyword in order to be changed.
- Constants should be written all uppercase with `_` as spaces.
- Shadowing is a way to change a variable and still make it immutable after
  the change is made.
- Shadowing also works to change the type of an immutable variable,
  does not seems too legit to me.
- The difference between signed and unsigned integres is that signed ones
  can hold negative numbers.
- Rust does not check for Integer Overflow when on release mode. Instead it
  wraps it to a number within the range of the size of the integer.
