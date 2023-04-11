# rb-15-4 Rc<T>.

## Things learned

- `Rc` stands for reference counting. The `Rc<T>` type keeps track
  of the number of references to a value to determine whether or not the
  value is still in use.
- We use the `Rc<T>` type when we want to allocate some data on the heap
  for multiple parts of our program to read and we can’t determine at
  compile time which part will finish using the data last.
- In terms of performance the `Rc::clone()` is not the same as a regular
  copy of data, in this case it only increments the reference counts to
  that data.
- `Rc::strong_count()` counts all the reference to the specified data.
