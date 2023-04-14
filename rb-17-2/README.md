# rb-17-2 OOP Traits edition.

## Things learned

- A difference between using a generic type parameter when defining
  a function on a trait and using a `trait object` is that in this code:

```rust
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

The components on the `Screen` struct all need to be of type `T`
whilst with a trait object it can be any time that implements
the `Draw` trait.
