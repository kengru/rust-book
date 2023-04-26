# rb-20 Multithreaded Web Server

## Things learned

- The two main protocols involved in web servers are Hypertext Transfer Protocol (HTTP)
  and Transmission Control Protocol (TCP).
- In networking, connecting to a port to listen to is known as “binding to a port.”
- When maping a vector we need to type it out. Ex: `Vec<_>`.
- The CRLF sequence can also be written as \r\n, where \r is a carriage return and
  \n is a line feed. The CRLF sequence separates the request line from the rest of
  the request data. Note that when the CRLF is printed, we see a new line start rather
  than \r\n.
- You can assign values with a let to the result of an if statement.
- `Vec.with_capacity()` preallocates space in the vector.
- For simplicity’s sake, this behavior is fine, but in a production thread pool
  implementation, you’d likely want to use std::thread::Builder and its spawn method
  that returns Result instead.
