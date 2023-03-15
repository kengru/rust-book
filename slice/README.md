# Slice

## Things learned

- If you are trying to iterate over strings to find a
  value, a useful way of doing so is to iterate over the bytes
  of the String.
- `b' '` <- this is a byte literal, it turns the space into
  its representation in bytes.
- `..` is used as a range.
- Slices are references to the value on the heap of the string,
  that means they too take the ownership away from the original
  variable when creating them.
- `[..]` references the entire array.
- Interger slices also exist.
