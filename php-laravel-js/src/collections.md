# Shape collections with iterators and closures

PHP's array can be a list or an associative map. In Rust, `Vec<T>` is a
growable list of one element type. `HashMap<K, V>` looks up values by key.
Choose `BTreeMap<K, V>` if you need iteration in key order. Use a struct
when the keys are fixed fields with different types.

`HashMap::entry` combines the lookup and the decision to insert or update.
This resembles counting with `$counts[$key] ?? 0` followed by an assignment.

```rust
use std::collections::HashMap;

let mut counts = HashMap::new();
for kind in ["email", "report", "email"] {
    *counts.entry(kind).or_insert(0_u64) += 1;
}

assert_eq!(counts.get("email"), Some(&2));
assert_eq!(counts.get("cleanup"), None);
```

`get` returns `Option<&V>`. Indexing can panic for a missing key or an
out-of-bounds position. For text, a `String` owns UTF-8 bytes and a `&str`
borrows them. `.len()` returns bytes, and `.chars()` visits Unicode scalar
values. For example, `"é".len()` is `2` while `"é".chars().count()` is `1`.
A user-perceived character may contain several scalar values. Use a
segmentation library if your application needs grapheme counts. Rust
requires byte ranges used to slice a `str` to land on character boundaries.

## Decide how the source is accessed

| Call on a `Vec<T>` | Item | Effect on the original vector |
| --- | --- | --- |
| `.iter()` | `&T` | Borrows it for reading. |
| `.iter_mut()` | `&mut T` | Borrows it exclusively and can edit each item. |
| `.into_iter()` | `T` | Consumes the owned vector. |

PHP's `array_map` and JavaScript's array `.map()` return a new array
immediately. Rust's `.map()` constructs a lazy iterator. `collect`, `count`,
`sum`, a `for` loop, and other consumers drive it. The destination type
often goes on `collect`, as in `collect::<Vec<_>>()`.

```rust
let rows = [" email ", "", "report"];
let queues: Vec<&str> = rows.iter()
    .map(|row| row.trim())
    .filter(|row| !row.is_empty())
    .collect();

assert_eq!(queues, ["email", "report"]);
```

A closure like `|row| row.trim()` can capture its surroundings. It borrows,
mutably borrows, or takes ownership according to how it uses each capture.
Use `move` when the closure must take captures by value, for example when a
thread needs to own a `String`. `FnOnce`, `FnMut`, and `Fn` describe how
the closure can be called. A closure that consumes a captured value may only
be callable once.

## Keep failures visible in pipelines

```rust
let ids = ["42", "invalid", "5"];
let parsed: Result<Vec<u64>, _> = ids.into_iter().map(str::parse).collect();
assert!(parsed.is_err());
```

Collecting an iterator of results into `Result<Vec<_>, _>` stops at the first
error. `filter_map(|raw| raw.parse::<u64>().ok())` goes ahead with the valid
entries and discards the malformed ones. The capstone fails on invalid
records and includes their line numbers, so its loop puts the failure path
next to the parsing step.

## Try it

Write two versions of “how many rows have kind `email`?” using a `for` loop
and `.filter(...).count()`. Then use `.iter_mut()` to trim a `Vec<String>`
by replacing each entry with its trimmed contents. Notice when you need an
owned string for the replacement. [Original collections chapter](https://rust-book.cs.brown.edu/ch08-00-common-collections.html) ·
[Original iterators and closures chapter](https://rust-book.cs.brown.edu/ch13-00-functional-features.html)

[Next — Use traits, generics, and lifetimes to design APIs](interfaces.md)
