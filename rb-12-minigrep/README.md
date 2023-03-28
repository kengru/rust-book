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
- This is a way of doing something when returning an error or an empty
  value from a function:

```rust
if let Err(e) = run(config) {
    println!("Application error: {e}");
    process::exit(1);
}
```

- Remember to add the `pub` keyword when separating functions into different
  files so they can be used in other parts of the program.
- _"Notice that we need to define an explicit lifetime 'a in the signature of search and use that lifetime with the contents argument and the return value. Recall in Chapter 10 that the lifetime parameters specify which argument lifetime is connected to the lifetime of the return value. In this case, we indicate that the returned vector should contain string slices that reference slices of the argument contents (rather than the argument query).
  In other words, we tell Rust that the data returned by the search function will live as long as the data passed into the search function in the contents argument. This is important! The data referenced by a slice needs to be valid for the reference to be valid; if the compiler assumes we’re making string slices of query rather than contents, it will do its safety checking incorrectly."_
- The `is_ok()` method with return true if a value exist or false if it doesn't.
  That makes sense when we are trying to see if an environment variable is setup at all.
