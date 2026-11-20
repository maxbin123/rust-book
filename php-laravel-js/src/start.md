# Rust for Laravel, PHP, and JavaScript developers

You already know how to ship a feature, define a service boundary, write a test,
and diagnose a failing deployment. You can go straight to Rust's differences.
If you can read a Rust function signature and run a Cargo project, start with
[ownership](ownership.md). If the syntax is new, spend one sitting on the
[Cargo and syntax refresher](cargo-and-syntax.md).

## What transfers

| When you reach for… | Start with… | Pay attention to… |
| --- | --- | --- |
| Composer or npm | Cargo | The same command-line tool drives dependency resolution, builds, tests, and examples. |
| A PHP DTO or validated JS object | `struct` | Field types describe what code can assume after parsing. |
| A JS object with a `kind` field | `enum` plus `match` | Each variant can carry different data. The compiler checks that every case is handled. |
| `null`, `undefined`, or `?->` | `Option<T>` | Absence is visible in the type. Methods such as `map` and `and_then` compose optional work. |
| A thrown exception or rejected promise | `Result<T, E>` and `?` | An expected failure is part of the return type. `?` can return it to the caller. |
| A Laravel service contract | A trait | A generic bound and a `dyn` trait object make different choices about dispatch and storage. |
| `array_map` or `rows.map(...)` | An iterator adapter | The iterator is lazy. Choosing `iter`, `iter_mut`, or `into_iter` determines who can use the items afterward. |
| `Promise.all` | A runtime's join operation | Calling a Rust `async fn` produces a future whose body runs when polled. |

The analogy stops where ownership begins. When a controller passes a PHP object
or JavaScript object to a function, both sides can retain access to the same
object. Passing an owned Rust `String` to a function can give that function
the value permanently. Accepting `&str` expresses temporary read access
instead. Read signatures with that distinction in mind throughout the book.

## What you'll build

The running example is a manifest of background jobs. A Laravel application
can export a description of jobs for reporting. The Rust side reads the file,
validates each record, and emits counts. You'll design the types, select which
inputs to borrow, attach context to errors, and test the library and CLI
separately. The project uses fictional records and runs without a Laravel app
or a queue backend.

Read through [tests](tests.md) for the core skills, then build the
[job audit CLI](job-audit.md). Read [storage and sharing](sharing.md),
[threads](threads.md), and [async](async.md) when choosing how a longer-running
program should handle work. Each chapter has a short exercise. For ownership
quizzes and visualizations, use the
[Cognitive Engineering Lab's interactive ownership chapter](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html).

[Next — Cargo and Rust syntax](cargo-and-syntax.md)
