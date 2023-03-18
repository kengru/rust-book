# rb-08-3 Hash Maps

## Things learned

- Hash map is like a dictionary in python or
  an object in javascript.
- On a hash map, all the keys have to be of the same
  typeand all the values have to be of the same type.
- To get a value from a hash map we use the `get` method
  which returns a `Option<&T>`, in order to get an option
  of values instead of references we can use the `copied`
  method and then to unwrap and set a default value in the
  case the options is `None`, the `unwrap_or(0)` can be used.
  `let score = scores.get(&name).copied().unwrap_or(0);`.
- Iteration works like on vectors but using key and value.
  When iterating, the order of the keys is arbitrary.
- The hash map implementation in Rust's standard library
  uses [SipHash](https://en.wikipedia.org/wiki/SipHash),
  it's secure but not the fastest, I have to keep in mind
  that in the future in case it's too slow for my use case.
- To overwrite a value you need to use the insert function
  with the same key.
- `entry` checks if a key has a value, if not, insert.
  `scores.entry(String::from("Yellow")).or_insert(50);`
