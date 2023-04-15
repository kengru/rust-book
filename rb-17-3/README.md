# rb-17-3 OOP Design Pattern.

## Things learned

- `self: Box<Self>` This syntax means the method is only valid
  when called on a Box holding the type. This syntax takes ownership
  of Box<Self>, invalidating the old state so the state value of the
  Post can transform into a new state.
- :smile:
- We call the as_ref method on the Option because we want a reference
  to the value inside the Option rather than ownership of the value.
  Because state is an Option<Box<dyn State>>, when we call as_ref, an
  Option<&Box<dyn State>> is returned. If we didn’t call as_ref, we
  would get an error because we can’t move state out of the borrowed
  &self of the function parameter.
