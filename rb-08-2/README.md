# rb-08-2 String

## Things learned

- To use the `to_string` method, we must implement the
  `Display` trait.
- To create a `String` from a string literal:
  `let s = String::from("initial contents");`.
- Strings have a push function called `push_str`.
- To concatenate several values its best to use the
  `format!` macro. With `+` we have to make sure we
  are using reference for all the values added but the
  first one, with `format!` we can just send the variables.
  `format!` returns a `String`.
- `String` does not support indexing. It has to do
  with strings being able to support UTF-8, which means some
  characters actually take more bytes than others.
- Slicing is possible but it looks like even that won't
  work that much when working with UTF-8 characters other than
  the ASCII ones.
- Iterating over a string needs to be well specified. Using
  `"Зд".chars()` will actually work and go throught each
  graphameme. `.bytes()` is also available.
- `contains` and `replace` are available, I need to read about
  them.
