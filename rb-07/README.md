# rb-07 Packages, Crates, Modules

## Things learned

- `Packages` can only have one `library` crate but
  many `binary` crates.
- A good reference to rules on modules can be found
  here: https://rust-book.cs.brown.edu/ch07-02-defining-modules-to-control-scope-and-privacy.html#modules-cheat-sheet.
- To create a package from `cargo` we can use the line:
  `cargo new <libraryname> --lib`.
- When making a package/project that has both a binary crate
  and a library crate, you have to make the things on the library
  crate public so they can be used in the main program.
  Remember to create good functions because you will use
  your own API.
- The `super` keyword can be used to access a parent's
  modules/functions.
- Structs can have individual fields being publics, `enums`
  can't.
- `pub struct Point(pub i32, pub i32);` <- makes sense but
  what the f.
- `use` puts a path into scope.
- When importing paths using `use` we can give them alias
  just like in javascript using `as`.
- `pub use` is some kind of import to export from a crate.
- We can use nested paths to bring the same items into scope
  in one line. We do this by specifying the common part of the
  path, followed by two colons, and then curly brackets around
  a list of the parts of the paths that differ.
  `use std::{cmp::Ordering, io};`.
- Another example: `use std::io::{self, Write};`.
- To bring all public fields/modules: `use std::collections::*;`.
