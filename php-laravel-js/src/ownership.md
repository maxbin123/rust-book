# Ownership, borrowing, and slices

A useful first question when writing a Rust function is what the caller will
want to do with the value after the call. A validator usually needs to read
it. A normalizer that edits a buffer in place needs exclusive access. A queue
that retains a job payload needs to acquire the payload or make its own copy.
Express that decision in the parameter type.

| Signature | What the caller grants |
| --- | --- |
| `fn send(payload: String)` | Ownership. The function can store, move, or drop the value. |
| `fn validate(payload: &str)` | A shared borrow of text. The caller keeps ownership. |
| `fn normalize(payload: &mut String)` | An exclusive borrow. The function can edit the caller's string. |
| `fn first_word(payload: &str) -> Option<&str>` | Shared access in, with a possible slice tied to the input. |

## Moves and copies

```compile_fail
fn enqueue(payload: String) {
    println!("{payload}");
}

let payload = String::from("report ready");
enqueue(payload);
println!("{payload}"); // use of moved value
```

The move transfers the `String` and bookkeeping for its heap allocation.
Rust treats the old binding as unusable, and the allocation can be released
once by the new owner. PHP's copy-on-write arrays and JavaScript's shared object
references suggest different behavior. Rust's `String` assignment or
by-value function call grants ownership to the destination.

Types that implement `Copy`, such as `u64`, can still be used after a by-value
call. For a `String`, an explicit `clone()` creates a second owned string
with independent storage. Choose it when two owners are what the design calls
for.

## Shared and exclusive access

```rust
fn first_word(payload: &str) -> Option<&str> {
    payload.split_whitespace().next()
}

fn add_marker(payload: &mut String) {
    payload.push_str(" [reviewed]");
}

let mut payload = String::from("weekly report");
let word = first_word(&payload);
assert_eq!(word, Some("weekly")); // last use of the borrowed slice
add_marker(&mut payload);
assert_eq!(payload, "weekly report [reviewed]");
```

`&str` is a borrowed slice of valid UTF-8. A slice is a view of some data. It
can cover the whole string or a portion of it. `&[T]` is the corresponding
read-only slice for contiguous elements, including a `Vec<T>`.

Rust allows multiple shared borrows or one exclusive borrow of the same data
at a time. Move the `assert_eq!` for `word` after `add_marker` and the shared
borrow would also need to remain valid during the call. Appending may
reallocate the buffer, so a held slice could point to data that moved. The
compiler prevents that overlap. This check follows uses of the borrow. In
the working version, `word` is finished before `add_marker`, even though both
bindings are in the same block.

## Fix the contract first

When you see “use of moved value,” ask whether the callee needs to retain the
value. Borrow if it only reads it. When you see “cannot borrow as mutable
because it is also borrowed as immutable,” look for the last use of the
shared borrow. Use it before mutation, narrow the scope, or extract an owned
result if it really needs to survive the change.

## Try it

Make `word` usable after `add_marker` by opting into a copy of its contents.
Compare that with the original ordering, where the word can stay borrowed.

```rust
let mut payload = String::from("weekly report");
let word = payload.split_whitespace().next().map(str::to_owned);
payload.push_str(" [reviewed]");
assert_eq!(word.as_deref(), Some("weekly"));
```

[`as_deref`](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_deref)
lets you compare the `Option<String>` as an `Option<&str>` while keeping the
owned value available. [Interactive ownership chapter and quizzes](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html)

[Next — Model jobs with structs, enums, and patterns](domain-modeling.md)
