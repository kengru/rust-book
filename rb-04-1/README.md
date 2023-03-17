# rb-04-1 Ownership

## Things learned

- `let a = Box:new(value);` creates a pointer to the _heap_.
- In a way, the stack holds the data of a function, while the heap holds
  data that can outlive a function.
- `let b = a;` if I then change the reference b is also pointing at,
  it will change the value on `a`.
- Rust does not permit manual memory management.
- Box deallocation principle (fully correct): If a variable owns a box,
  when Rust deallocates the variable's frame, then Rust deallocates the box's
  heap memory. Which is to say, if a variable is no longer needed and will
  not be on scope, and it is the last owner of a pointer, then said heap
  value asociated with that pointer will also deallocate.
- Collections use Boxes (`Vec`, `String` and `Hashmap`). Mostly because
  they need to use the heap to hold a variable number of elements.
- Ok so, if after transfering the ownership of a heap allocation I modify
  the value in the heap, the original heap memory gets freed and the original
  variable can no longer be used. Ok.
- It's important to notice that the moved heap behavior does not work the same
  for every data type.
- By cloning a variable we are creating a _"deep"_ copy instead of a _"shallow"_
  copy.
- _"ownership is a discipline of pointer management."_
- [Ownership Summary](https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html#summary) in case
  I need to refresh.
