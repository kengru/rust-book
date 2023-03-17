# Structs

## Things learned

- `Structs` are basically classes.
- To get an "object" we instanciate the struct. If a variable
  has the same name as a field, we don't have to repeat it, that is
  called `"field init shorthand"`.
- You can do the same thing as in javascript where you can add
  all the values from another struct into the first one:

```rust
let user2 = User {
  email: String::from("another@example.com");
  ..user1
};
```

- Keep in mind that when using struct update syntax (`..`) we
  are passing ownership on values that deal with the Heap (like
  Strings), which means the object we are copying from can no longer
  be used. (I think)
- `Tuple structs` are like small clases that look like a tuple of
  length x, they seem usable: `struct Point(i32, i32, i32)`. This is
  basically like a named tuple.
- Even if two tuple structs have the same type, they cannot be used
  interchangably in things like parameters of a function, this makes
  sense.
- There is also a thing called Unit-like structs which are going to
  be explained later.
- Ok, I think I got it, it's like a class that will not have data
  saved but will have some functions implemented.
- To store non owned values like `&str` in a struct, requires something
  called lifetimes.
