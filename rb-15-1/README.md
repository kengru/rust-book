# rb-15-1 Box<T>.

## Things learned

- `String` and `Vec<T>` are both smart pointers, which means that
  besides the reference to the heap they also have metadata and actually
  own the data in the heap instead of borrowing it as we do with normal
  references.
- The `Deref` trait allows an struct to behave like a reference.
  The `Drop` trait allows you to customize the code that's run when an
  instance of the smart pointer goes out of scope.
- A `Box<T>` allows you to save data on the heap instead of the stack.
- When moving ownership of a large amount of data, it's best not to
  copy everything like we would do on the stack but just use a Box and
  transfer the ownership of the heap data.
- In a 64-bit architecture a the size of a pointer is 8 bytes.
