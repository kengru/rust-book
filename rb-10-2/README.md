# rb-10-2 Traits

## Things learned

- Traits define functionality of a particular type.
- Traits mainly define an interface, which is an
  implementation a type could have. They don't define
  the behaviour, its more like the end result.
- You can actually define a default behaviour too.
- We can use traits to define different function
  implementations into a same thing and then add them
  into a struct. If you don't expecify the behaviour
  of the trait function you are implementing then
  you need to add it when adding it to a struct.
  If you do, then it acts like a default implementation.
- You can create a function where the parameters that it takes
  is of type "something that implements a trait". Examples:

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

and

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

- `pub fn notify(item: &(impl Summary + Display))` is a
  case where we want the `item` value to implement both
  Summary and Display.
- _"Traits and trait bounds ensure that even though the
  types are generic, they’ll have the behavior the code needs."_
