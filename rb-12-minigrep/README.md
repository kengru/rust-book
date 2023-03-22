# rb-12 Minigrep

## Things learned

- `std::env` contains a function called args that returns an
  iterator over the arguments passed to the program.
- The `collect()` function turns an iterator into a collection,
  in the case of the arguments it can turn it into a Vector of Strings.
  Collect is also one of the few functions that we do need to explicitely
  annotate its types.
