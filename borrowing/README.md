# Borrowing

## Things learned

- A reference is a non-owning pointer, which means after sending it
  out of scope via a function for example, the ownership of the heap
  value stays in the original variable. Looks like this: `&variable`.
- Creating a reference is what it's called _"borrowing"_.
- To access a pointer's data we use the "dereference" operator which
  is `*`.
- Aliasing is accessing the same data through different variables.
- When using a `Vec`, everytime you push an element to the array, it
  is potentially deallocating the original heap array and putting it
  in another memory location. This also means that if you have a
  pointer to a value inside the Vec, whenever you push an item to the
  Vec, the pointer will no longer be working.
- In theory, the compiler checks if variables have either one of these
  three permissions: Read/Write/Own. References can temporarily remove
  these permissions.
- _"Pointer Safety Principle: data should not be aliased and mutated."_
- The Rust compiler does so many checks man.
- Trying to write on a variable that has been borrowed while there are
  still more uses to the borrowing variable is ilegal.
- You can also change this ^ by making the borrowing variable mutable:
  `let num: &mut i32 = &mut vec[2];`.
- Permissions are returned at the end of a reference's lifetime, which
  mean the after the last place it was used.
- The `ascii_capitalize` example in `main.rs` explain a bit more of
  what its meant by lifetime.
- _"Data must outlive any references to it."_
- Variables or references created in a stack like in a function cannot
  be used back on main.
