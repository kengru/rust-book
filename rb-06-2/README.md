# rb-06-2 Match

## Things learned

- The important part about `match` is that the compiler
  can make sure that all patterns that can come out of
  the evaluation of the data are handled.
- `match` is kind of like a switch in javascript.
- When out of an option you need to get any kind of type
  we are better of using `match` than a simple `if`.
- If needed it, you can treat the arms of a `match` like
  a function using `{}`. Also, the match function is of
  course an expression, so it returns something.
- On a `match` arm you can access values from the enum
  in question by using a funtion signature.
- `match` can be use in almost every case, even for example
  the value of an integer variable.
- If an arm is used to handle every other case, we
  can use `other` as the pattern option, if we don't want
  to use the value we use `_`.
- Remember that the unit value for Rust is an empty tuple:
  `()`.
- As always, if the value uses data from the Heap, we should
  worry about ownership.

### if let

- You can think of `if let` as syntax sugar for a match that runs
  code when the value matches one pattern and then ignores all
  other values.
