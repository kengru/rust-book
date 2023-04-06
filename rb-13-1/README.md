# rb-13-1 Closures.

## Things learned

- Rust’s closures are anonymous functions you can save in
  a variable or pass as arguments to other functions.
- Like anonymous closures in javascript you can take no parameters
  and just return a value.
- Closures can be anotated:
  `let add_one_v2 = |x: u32| -> u32 { x + 1 };`
- The Rust compiler will infer that a Closure's parameter types
  are the ones first used, so any other types used later will cause
  a compiling error.
- The toilet closure is similar to std::mem::drop, i.e. a function
  that moves an argument and causes it to be dropped.

Toilet closure:

```rust
let f = |_| ();
```

- If you need a new thread to have ownership of some data we can
  use `move` with closures. Example:

```rust
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
```
