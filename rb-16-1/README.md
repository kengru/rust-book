# rb-16-1 Simultaneous Code.

## Things learned

- Code is usually run in a `process`.
- There is no inherent guarantee about the order in which parts
  of your code on different threads will run.
- `thread::sleep()` can be used to set a thread, spawned or
  main, to sleep for a duration.
- The `Duration` struct has some capabilities to express time
  in Rust.
- A `JoinHandle` is an owned value returned from creating a thread
  using `thread::spawn()`. If you call the `join()` method on it,
  it will wait for the thread to finish before the `main` thread exists.
  It does so by blocking the main thread.
- Values created outside of the thread spawned cannot be used unless
  the ownership is moved to it.
- `move` can be used to move ownership of the values on a thread to the
  one just spawned. Remember that if the ownership is moved, the value
  cannot be used unless the ownership returns.
