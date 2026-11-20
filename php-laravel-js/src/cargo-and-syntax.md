# Cargo and Rust syntax in one sitting

Use a stable Rust toolchain (1.90 or newer for the examples). Install it with
[rustup](https://rustup.rs/) if needed. The code is Rust 2024, an edition set
per package in `Cargo.toml`. An edition controls language rules. The toolchain
version identifies the compiler and standard library.

Create a scratch package outside this repository for the chapter examples.

```sh
cargo +stable new --edition 2024 job-sandbox
cd job-sandbox
cargo +stable check
cargo +stable run
cargo +stable fmt --check
```

`check` type-checks without producing the executable. `run` builds and launches
the binary, and arguments after `--` go to that binary. `cargo test` compiles
and runs tests. `cargo clippy` provides additional lints when the Clippy
component is installed.

`Cargo.toml` holds package metadata and dependency requirements, like
`composer.json` or `package.json`. `Cargo.lock` records the resolved versions.
Commit it for this CLI so a fresh checkout can use `cargo build --locked`.
Cargo builds in a development profile by default. Use `cargo run --release`
when measuring performance, since the optimized build can behave very
differently in benchmarks.

## Read signatures and expressions

```rust
fn retry_delay(attempts: u32) -> Option<u64> {
    let seconds = match attempts {
        0 => 5,
        1..=3 => 30,
        _ => return None,
    };
    Some(seconds)
}

assert_eq!(retry_delay(2), Some(30));
assert_eq!(retry_delay(4), None);
```

`attempts: u32` is an unsigned 32-bit integer. `-> Option<u64>` means callers
get either a 64-bit unsigned value or `None`. Rust checks both sides of the
contract. It also infers many local types, including the type of `seconds`
from its use in `Some(seconds)`.

The final expression `Some(seconds)` supplies the return value. Adding a
semicolon there turns it into a statement and leaves the block with the unit
value `()`. `if` and `match` can produce values. An `if` condition must be
`bool`, so spell out checks such as `!jobs.is_empty()` and `count > 0`.

`let` is immutable by default. `let mut` enables assignment or a mutable
borrow, while a second `let` can shadow the old binding and change its type.
Shadowing is handy when turning unvalidated text into its parsed form.

```rust
let job_id = "42";
let job_id: u64 = job_id.parse().expect("hardcoded id should be valid");
assert_eq!(job_id, 42);
```

Rust has fixed-width integers such as `u64` and `i32`. Use `usize` for
in-memory lengths and indexing, and a fixed-width type when the width matters
to a data format. Numeric conversions are explicit. `u32::try_from(id)` is a
checked narrowing conversion, while `id as u32` can truncate a wider integer.
Text literals are `&str`. Use `String` when you need owned, growable text.

`!` in `println!`, `assert_eq!`, or `vec!` marks a macro call. `::` qualifies a
name on a path or type, as in `String::new()`. A dot calls a method on a value,
as in `name.trim()`.

## Try it

Paste `retry_delay` into your sandbox and call it from `main`. Then change the
example so a retry delay is available only for attempts `0` through `2` and
run the assertions again. Let `cargo check` show what happens when you add a
semicolon after `Some(seconds)`. [Original Cargo chapter](https://rust-book.cs.brown.edu/ch01-03-hello-cargo.html)

[Next — Ownership, borrowing, and slices](ownership.md)
