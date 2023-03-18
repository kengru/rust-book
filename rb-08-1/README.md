# rb-08-1 Vectors

## Things learned

- Vectors are used to save data in memory of the same type.
- To create a vector we can call the new method on `Vec`:
  `let v: Vec<i32> = Vec::new();`
- The `vec!` macro is used to initialize a vector with some
  value: `let v = vec![1,2,3];` they type of v is infered
  if there is a value assigned.
- To be able to push a value to the vector we need to use
  the push method in the vector and make the variable that
  holds it mutable.
- When trying to access data in a vector, the difference
  between using `[]` or the `get` method is that the brackets
  will make the program panic if the index is out of bounds
  but get will actually just return a `Option<T>`, which can
  be then catched inside a `match` pattern matching, in case
  it is out of bound, it will just return None.
