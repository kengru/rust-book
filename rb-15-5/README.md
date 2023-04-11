# rb-15-5 RefCell<T>.

## Things learned

- `unsafe` is a word ment for code that is not going to be checked
  by the compiler, but in runtime.

Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:

- `Rc<T>` enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
- `Box<T>` allows immutable or mutable borrows checked at compile time; Rc<T> allows
  only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable
  borrows checked at runtime.
- Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the
  value inside the RefCell<T> even when the RefCell<T> is immutable.
