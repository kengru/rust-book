# rb-16-2 Messaging.

## Things learned

- The way to communicate between threads is by using channels.
- `mpsc` stands for _multiple producer single consumer_.
- The `send` on the transmitter takes ownership of the value
  it sends.
- If on a loop to get data, the loop stops when the thread
  stops.
- Channels can only send values of a single type, so tx.send(s.len())
  is a type error. If you want to send values of multiple types, you
  can use either an enum or the Any trait.
