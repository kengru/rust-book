# rb-19-1 Unsafe Rust

## Things learned

- To switch to unsafe Rust, use the unsafe keyword and then start a new
  block that holds the unsafe code. You can take five actions in unsafe Rust
  that you can’t in safe Rust, which we call unsafe superpowers. Those superpowers
  include the ability to:

- Dereference a raw pointer
- Call an unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe trait
- Access fields of unions

- An FFI (Foreign Function Interface) is a way for a programming language to define
  functions and enable a different (foreign) programming language to call those functions.
- Mangling is when a compiler changes the name we’ve given a function to a different name
  that contains more information for other parts of the compilation process to consume but
  is less human readable.
