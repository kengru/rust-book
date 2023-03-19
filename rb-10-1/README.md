# rb-10-1 Generics

## Things learned

- It's important to recognize when code duplication is
  happening. Generics help with that. They work just
  like in typescript.
- By convention the generic parameter name is `T`.
- When using generics we need to keep in mind what
  traits the types have, if we need something like
  comparing, the `PartialOrd` trait must be implemented
  by the type.
- Struct example:

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

- We can implement methods to work on an specific type
  even if the struct we are implementing it has a generic
  type.
- In practice, Rust creates the actual functions, structs
  or enum that are utilized with the generics.
- With generics in theory we cannot even use it as a string,
  printing it for example.
