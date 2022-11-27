# Chapter 02

## Things learned

- To read user input we need the `std::io` library, and then use `io::stdin`.
- Variables are inmutable by default. To make a variable mutable we add `mut` before the variable name: `let mut bananas = 5;`.
- Strings are created by returning a new instance of the `String` type: `let mut new_string = String::new();`
- An associated function is a function that's implemented on a type. Something like a method of a class.
- When handling input with Stdin, we need to pass a mutable variable for it to **append** the input contents.
- References are immutable by default.
- A function like `read_line` returns a `Result` value, which is an enum that has **variants** `Ok` and `Err`. This is just like a response from a function that does not need to respond something specific but just if there was an error on the operation or not.
- "Template literals" here are just interpolation with curly brackets. I need to check how to scape those brackets later.
- There is a difference between binary crates and library crates, binaries create an executable, library just have code intented to be used by other programs.
- You can "shadow" a mutable variable by just redeclaring it, useful for times where we are just trying to parse a value.
- `match` works like an multiple arms. Can check if a function has some conditions and then just runs the one that is true.
- `match` is great!

## Ideas

- I am not quite sure why but I got the idea of whatever tool I'm planning to create with Rust should start with a motivational video.

## Thoughts

- I really need to learn vim. Will be forcing myself to do it.
