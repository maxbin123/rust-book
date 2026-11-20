# Make errors part of the contract

Laravel often converts a validation exception to a response at the edge of a
request. Keep that separation when writing Rust. A parser returns structured
information about invalid input. A CLI decides what to print and which exit
code to return. A web handler could turn the same failure into an HTTP status.

`Option<T>` describes an absent value without a reason. `Result<T, E>`
describes a successful value or a particular failure. Give invalid input a
reason when the caller can act on it. This example distinguishes a missing
field from a present field with an invalid ID.

```rust
use std::num::NonZeroU64;

#[derive(Debug, PartialEq, Eq)]
enum IdError {
    Missing,
    Invalid,
}

fn parse_id(raw: Option<&str>) -> Result<NonZeroU64, IdError> {
    let raw = raw.ok_or(IdError::Missing)?;
    raw.parse().map_err(|_| IdError::Invalid)
}

assert_eq!(parse_id(Some("42")).map(NonZeroU64::get), Ok(42));
assert_eq!(parse_id(None), Err(IdError::Missing));
assert_eq!(parse_id(Some("0")), Err(IdError::Invalid));
```

`ok_or` turns `None` into an error. On `Ok`, `?` extracts the value and
continues. On `Err`, it returns from the surrounding function, converting
the error when the return type supports conversion with `From`. It does
similar early return with `None` inside an `Option`-returning function.
`map_err` lets you choose a conversion explicitly. Here the parser translates
all failed integer parses to `Invalid`, which is sufficient for this caller.

## Return enough context

A reusable parser can return a kind of failure such as `Invalid`. The file
reader knows the line number. The binary knows the filename and can format
`sample.jobs: line 7: invalid job id` on stderr. Keeping context at the
layer where it is available makes the library usable independently of the CLI.
Implement `Display` for messages aimed at people and `std::error::Error` to
integrate with Rust's error APIs. Implement `source()` when the error wraps
a lower-level cause worth inspecting.

At an application boundary, `Box<dyn std::error::Error>` can be a convenient
return type when callers only need to report failures. An error enum makes
more sense at a library boundary where callers match on particular failures.
A [`main` that returns `Result`](https://doc.rust-lang.org/std/process/trait.Termination.html)
can handle a small tool's basic reporting. The capstone uses `ExitCode` to
give usage errors and processing failures distinct codes and format its own
diagnostics.

Reserve `expect` for a condition that the code treats as an invariant, as in
parsing a hardcoded fixture. For malformed CLI input, return an error the
caller can display. With unwinding enabled, an uncaught panic terminates
the current thread. With abort configured, a panic aborts the process.
`unwrap()` and `expect()` panic on `None` or `Err`.

## Try it

Write `parse_optional_id(raw: Option<&str>) -> Result<Option<NonZeroU64>, IdError>`.
Treat a missing ID as allowed and reject invalid IDs. Add cases for absence,
`"42"`, and `"0"`. `Option::transpose` can flip `Option<Result<T, E>>` into
`Result<Option<T>, E>`. [Original error handling chapter](https://rust-book.cs.brown.edu/ch09-00-error-handling.html)

[Next — Organize a Cargo package](packages.md)
