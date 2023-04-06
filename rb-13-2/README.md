# rb-13-1 Iterators.

## Things learned

- Iterators in Rust are lazy, that means they don't do
  anything unless called to do something.
- Everytime you are using an iterator you are consuming
  the values. A for loop, for example, takes ownership of the
  iterator.
- In the case we need an iterator that takes ownership of the
  list/vector/array and returns owned values instead, we can use
  `into_iter` instead of `iter`. In the case of mutable references
  we can use `iter_mut`.
- A _consuming adaptor_ is a method that calls the `next` method
  of an iterator, because calling it uses up the iterator.
- _Iterator adaptors_ are methods that don't consume the iterator.
  Instead, they produce different iterators. An example of this is
  `map`. `map` in Rust returns a new iterator.
- To get back a collection we can call the `collect()` method on
  an iterator.
