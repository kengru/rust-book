# rb-17-1 OOP.

## Things learned

- Rust is not designed as an object oriented programming language
  but it does have capabilities that allow you to use the design
  practices of it. For example, `structs` are made as object and
  the `impl` block creates methods for them.
- Encapsulation in Rust is default since the way to use a method
  or even a struct from a library is through the `pub` keyword, otherwise
  they remain unreachable by the main code trying to use it.
- Adding pub here makes the struct public but not its fields:

```rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: i64,
}
```

- Remember to use `&mut self` when changing the self inside a method.
- Traits are a way to implement inheritance within "classes" (structs
  in Rust case).
