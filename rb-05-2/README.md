# rb-05-2 Rectangle

## Things learned

- The reason we borrow the values when sending a struct, ie
  sending only the reference, it's because that way ownership
  is not lost and the original variable can still be used.
- `println!` tries to format `{}` by accesing the Display
  method of the value. By default primitive types have it.
- `#[derive(Debug)]` implementing that into a struct (basically
  putting that line above the struct definition, like a decorator)
  makes us able to print the data of the instanciated struct in a
  println! by using `{:?}`.
- `dbg!` is another macro that works as a printer but in the standard
  error console stream. It works great because it does not take
  ownership away from the variable and it does return the result.
