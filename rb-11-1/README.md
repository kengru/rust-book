# rb-10-1 How to write tests.

## Things learned

- Tests are run with the `cargo test` commands.
- A test is a function that has the `#[test]` attribute
  on top of it.
- Use `assert_eq!()` when comparing two values and `assert!()`
  when trying to test a boolean condition. `assert_ne!()` also
  works.
- When creating new structs or enums, they should implement
  the `PartialEq` and `Debug` traits, which sometimes is as
  easy as just deriving them onto the struct/enum:
  `#[derive(PartialEq, Debug)]`.
- You can add custom messages to the asserts.
- The `#[should_panic]` derivative does what the name says,
  checks if it the test itself panics due to the calling of the
  tested function.
