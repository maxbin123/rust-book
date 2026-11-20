# Advanced Rust and further reading

The capstone gives you a reusable library and thin executable. Extend
that boundary when you have a reason to take on a more advanced feature.
These topics are starting points for reading Rust libraries and extending
the project.

## Give domain values and interfaces sharper types

A type alias names an existing type. A newtype wraps an existing type and
creates a distinct one. It is useful when two IDs have the same underlying
representation and should stay distinct at the call site.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct JobId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AccountId(u64);

fn lookup_request(job: JobId, owner: AccountId) -> (u64, u64) {
    (job.0, owner.0)
}

assert_eq!(lookup_request(JobId(42), AccountId(5)), (42, 5));
```

`Iterator::Item` is an associated type. Each implementation chooses the
type produced by that iterator. Trait bounds can refer to it, for example
`impl Iterator<Item = JobId>`. Rust's coherence rules generally require a
trait or relevant type to be local to your crate to write an implementation.
Wrapping an external type in a local newtype can create a place to implement
an external trait.

An `fn` pointer can hold a function or a compatible closure that captures
nothing. A generic `F: Fn(...)` can accept a closure that captures state.
Returning `impl Fn(...)` lets you return one concrete closure type while
hiding its name. Use a boxed trait object if the returned callbacks need
different underlying types.

## Read macros, FFI, and unsafe code at their boundaries

`macro_rules!` macros match and emit syntax. Procedural macros consume and
produce token streams, including derives such as `#[derive(Debug)]` and
attributes such as `#[tokio::main]`. Use a regular function for behavior
that can be expressed in terms of values and a macro when you need to
generate syntax.

Foreign function interfaces and some low-level library internals call
for `unsafe`. An `unsafe` block permits specific operations such as
dereferencing a raw pointer and relies on the author to uphold their
safety requirements. Normal type and borrowing rules still apply. Read
the safety contract at the call site and keep the unchecked operation
behind a small safe interface where possible.

## Try it

Wrap the capstone's `NonZeroU64` in a `JobId` type and update the parser
and tests. Then try composing a test-only audit sink with a production
sink to practice the choice between a trait bound and a trait object.

## References

- Use the ownership inventories and design exercises for more practice. [Interactive ownership chapter](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html) · [Object-oriented design and trade-offs](https://rust-book.cs.brown.edu/ch18-00-oop.html)
- Read the advanced chapters when you need lifetimes, trait rules, macros, or unsafe code in detail. [Original advanced features chapter](https://rust-book.cs.brown.edu/ch20-00-advanced-features.html) · [Rust Reference](https://doc.rust-lang.org/reference/) · [Rustonomicon](https://doc.rust-lang.org/nomicon/)
- Look up keywords, operators, derivable traits, tooling, editions, translations, and release channels as needed. [Original appendices](https://rust-book.cs.brown.edu/appendix-00.html)
