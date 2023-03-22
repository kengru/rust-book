# rb-12 Minigrep

## Things learned

- `std::env` contains a function called args that returns an
  iterator over the arguments passed to the program.
- The `collect()` function turns an iterator into a collection,
  in the case of the arguments it can turn it into a Vector of Strings.
  Collect is also one of the few functions that we do need to explicitely
  annotate its types.
- The `std::fs` package on the standard library has some functions to
  work with files.
- The `unwrap_or_else` function is great for when we are trying to get
  a value from a `Result` and the result returns Err. We can write a
  closure to the parameters of the function and handle the error within
  the closure's scope, probably ejecting the programm with an exit code
  other than 0.
