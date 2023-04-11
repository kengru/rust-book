# rb-15-6 Reference Cycles.

## Things learned

- Reference cycles are bad because they can leak memory.
- You can also create a weak reference to the value within
  an Rc<T> instance by calling Rc::downgrade and passing a
  reference to the Rc<T>. Strong references are how you can
  share ownership of an Rc<T> instance. Weak references don’t
  express an ownership relationship, and their count doesn't
  affect when an Rc<T> instance is cleaned up. They won’t cause
  a reference cycle because any cycle involving some weak references
  will be broken once the strong reference count of values involved is 0.
- Thinking about the relationships another way, a parent node should
  own its children: if a parent node is dropped, its child nodes should
  be dropped as well. However, a child should not own its parent: if we
  drop a child node, the parent should still exist. This is a case for weak
  references!
- If we just need to refer to a node, we can use a weak reference, this
  will avoid ownership problems.
