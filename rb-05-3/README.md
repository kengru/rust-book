# rb-05-3 Methods

## Things learned

- Methods always have the `self` parameter, it represents
  the current instance of the struct.
- `impl` is how implementations for a struct is made.
- _"Having a method that takes ownership of the instance by
  using just self as the first parameter is rare; this technique
  is usually used when the method transforms self into something
  else and you want to prevent the caller from using the original
  instance after the transformation."_
- It is possible to write a method that it's called like a
  field.
- Always remember that if a function is just going to read the
  values of something like a struct, it's better to do an
  immutable borrow of the value. `&rect1` for example.
- All functions defined within an `impl` block are _associated
  functions_
- `impl` can be called inside the main function (why?).
