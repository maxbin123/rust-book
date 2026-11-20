# Run work on threads

Running two Laravel queue workers gives you two processes. A Rust program
can run work on multiple OS threads in one process, with an explicit choice
about which data the threads share. A worker can own its inputs and send
results back. Shared immutable settings can be borrowed for scoped work or
owned through an `Arc` for longer-lived work.

## Move values through a channel

```rust
use std::sync::mpsc::sync_channel;
use std::thread;

let (sender, receiver) = sync_channel::<String>(2);
let worker = thread::spawn(move || {
    for line in ["email|42|order shipped", "report|99|weekly metrics"] {
        sender.send(line.to_owned()).expect("reader should still be running");
    }
});

let rows: Vec<String> = receiver.into_iter().collect();
worker.join().expect("manifest worker should finish");
assert_eq!(rows.len(), 2);
```

`move` gives the closure ownership of the sender. `send` takes its message
by value and transfers it to the channel. Iterating the receiver waits for
messages and ends after all senders have been dropped and all buffered
messages have been read. A bounded `sync_channel` blocks the sending thread
when capacity is full. That gives a producer backpressure if the reader
falls behind. Drain the receiver while the producer runs. Joining the
producer first can deadlock when it fills the channel.

`thread::spawn` requires captures that can meet a `'static` bound because
the spawned thread may outlive the caller. An owned `String` can meet that
bound even though it is freed after use. Use `thread::scope` when threads
should finish before the surrounding function proceeds and need to borrow
its data.

```rust
use std::thread;

let rows = vec![String::from("email"), String::from("report")];
let total_bytes = thread::scope(|scope| {
    let left = scope.spawn(|| rows[0].len());
    let right = scope.spawn(|| rows[1].len());
    left.join().unwrap() + right.join().unwrap()
});
assert_eq!(total_bytes, 11);
```

## Read thread-safety bounds

`Send` means ownership can cross a thread boundary. `Sync` means shared
references can cross that boundary, equivalently `&T: Send`. A type's fields
usually determine these properties automatically. An `Rc<T>` uses a
non-atomic reference count and stays on one thread. An `Arc<T>` can be sent
and shared when `T` meets its required `Send` and `Sync` bounds. A `Mutex`
can coordinate independent owners that need to change the same data. Keep
its guard scoped to the operation that requires the lock.

For a worker shutdown, stop producing, drop the senders, drain the receiver,
and join the worker threads. An in-memory channel coordinates one process.
A Laravel queue broker also provides cross-process delivery and whichever
persistence and retry behavior its backend and configuration support.

## Try it

Change the channel capacity to zero and run the example. Each send now
rendezvous with a receive. Look at the order of `collect` and `join` and
trace what would happen if you swapped them. [Original threads and concurrency chapter](https://rust-book.cs.brown.edu/ch16-00-concurrency.html)

[Next — Use async when work spends time waiting](async.md)
