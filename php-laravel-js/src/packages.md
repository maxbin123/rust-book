# Organize a Cargo package

A Composer package can expose namespaced classes and install a CLI command.
A Cargo package can similarly expose a library and build a binary. Cargo
and the compiler use more specific names for the pieces.

| Rust term | What it means for `job-audit` |
| --- | --- |
| Package | The unit described by `Cargo.toml`. |
| Crate | One compilation unit. `src/lib.rs` and `src/main.rs` are roots of two different crates in the same package. |
| Module | A namespace and privacy boundary within a crate, declared with `mod`. |
| Workspace | A set of packages managed together, generally with a shared lockfile and target directory. |

## Keep the executable thin

```text
job-audit/
  Cargo.toml
  Cargo.lock
  src/
    lib.rs
    main.rs
  sample.jobs
```

`src/lib.rs` is the library entry point. Put parsing and counting there.
`src/main.rs` reads arguments, opens the file, and calls the library with
`use job_audit::audit;`. The crate name uses an underscore where the package
name has a hyphen.

If parsing grows, put it in `src/parse.rs` and declare `mod parse;` from
`lib.rs`. Files do not become modules on their own. `use` brings a name
from the module tree into scope. `pub use` re-exports it, which lets the
library offer a stable path while moving code between its private modules.

```rust
mod parse {
    pub fn queue_label() -> &'static str {
        "email"
    }
}

pub use parse::queue_label;

assert_eq!(queue_label(), "email");
```

Items are private by default. `pub(crate)` makes an item visible inside the
current crate. To use an item from `main.rs`, the binary needs access to it
through the library's public API, even though the library and binary are
built from one package. `crate::` addresses your own crate root, `super::`
the parent module, and `self::` the current module.

## Use Cargo for delivery

`cargo add` adds a dependency requirement to `Cargo.toml`. Inspect resolved
dependencies with `cargo tree`, including enabled features with
`cargo tree -e features`. Features often opt into derives or runtime support.
For an application release, run `cargo build --release --locked` so the
build uses the checked-in dependency resolution. `cargo install --path .`
installs a package's binary locally. Publishing a reusable library to
crates.io has its own metadata and release workflow.

Use a workspace when related packages benefit from one build and dependency
resolution, such as a domain crate and separate CLI and service crates. Start
with a single package and modules when all you need is separation of code.
That is the layout used by the capstone.

## Try it

Split a `retry_delay` function into `src/policy.rs` in your sandbox binary.
Declare `mod policy;` in `main.rs` and call `policy::retry_delay(...)`.
Try `pub(crate)` and then remove the visibility qualifier to see what Rust
checks across the module boundary. [Original modules chapter](https://rust-book.cs.brown.edu/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) ·
[Original Cargo chapter](https://rust-book.cs.brown.edu/ch14-00-more-about-cargo.html)

[Next — Shape collections with iterators and closures](collections.md)
