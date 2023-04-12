# rb-16-3 Shared-State Concurrency.

## Things learned

- Message passing is a fine way of handling concurrency, but it’s
  not the only one. Another method would be for multiple threads to
  access the same shared data.
- Mutex is an abbreviation for mutual exclusion, as in, a mutex allows
  only one thread to access some data at any given time. To access the
  data in a mutex, a thread must first signal that it wants access by
  asking to acquire the mutex’s lock.
- The type of m is Mutex<i32>, not i32, so we must call lock to be able
  to use the i32 value. We can’t forget; the type system won’t let us
  access the inner i32 otherwise.
- Locks are released at the end of the scope.
