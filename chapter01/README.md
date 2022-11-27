# Chapter 01

## Things learned

- `rustc {filename}` compiles a rust file and creates the binary in the folder you are at.
- `cargo new {project_name}` creates a new rust package, that is, a new folder with a `Cargo.toml` file and a `main.rs` file inside a `src` folder.
- The `Cargo.lock` files works just like a `package-lock.json`.
- If a function as a `!` at the end, for example `println!` it means it's a Rust macro and not a function. I do not know what this means yet.
- `cargo build` builds a package just like `rustc` would but easier. It saves the binary on `./target/debug/{package_name}`.
- `cargo run` can build and run a package at the same time.
- `cargo check` verifies the package will compile without creating an executable.
- When trying to release a package, build it with `cargo build --release`, the executable will be on `./target/release/...`.
- To update Rust, we can use `rustup update`.
