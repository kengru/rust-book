# rb-14 Publishing crates.

## Things learned

- When compiling code with cargo, you can use profiles to
  determine things like the optimization level that rust will
  use. This is done by adding the `[profile.*]` section in the
  `Cargo.toml` file.
- `///` is used to document functions. Everything after those
  is formatted using markdown.
- To open the docs for your crate, we use `cargo doc --open` the
  resulted files from the docs will be on `target/doc`.
- Things to document about functions/library when creating a
  public API: Examples, Panics, Errors and Safety.
- `//!` are used to describe the crate.
- Any example written in the documentation will cound as a test
  when running `cargo test`.
- Crate versions cannot be overwritten. In case of an error just
  public a new version and `yank` the old one.
