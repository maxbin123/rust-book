# Test the contract

Bring the same boundary thinking you use in Pest, PHPUnit, or Vitest. A
parser test should exercise text and check typed output. An integration
test should call the public API. A command-line smoke test should use a
real invocation and observe stdout, stderr, and the exit status.

## Unit, integration, and documentation tests

Place unit tests beside the code in a `#[cfg(test)] mod tests`. They can
use private helpers in the parent module through `use super::*;`. Use
files in `tests/` to exercise public behavior as another crate would.
Rustdoc compiles Rust examples in documentation comments and runs them
by default. `mdbook test php-laravel-js` checks the examples in this book.

```rust
use std::num::NonZeroU64;

fn valid_id(raw: &str) -> bool {
    raw.parse::<NonZeroU64>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zero_as_a_job_id() {
        assert!(!valid_id("0"));
    }
}
```

Use `assert_eq!` for exact values, `assert!` for predicates, and pattern
matching to inspect variants with extra fields. An `assert_eq!` needs
`PartialEq` to compare and `Debug` to report differences on failure.
When testing an enum that wraps `std::io::Error`, compare the outer
variant and inspect `source.kind()` instead of comparing whole errors.

For the capstone, `audit(input: impl BufRead)` can take a `BufReader<File>`
in production and an in-memory byte slice in a test. Injecting input at
this boundary gives cheap tests without building a mock filesystem.
Likewise, an error-returning parser enables tests of error variants
without checking message formatting everywhere.

## Run the tests you need

```sh
cargo +stable test --manifest-path php-laravel-js/job-audit/Cargo.toml
cargo +stable test --manifest-path php-laravel-js/job-audit/Cargo.toml reports_physical_line_number
cargo +stable test --manifest-path php-laravel-js/job-audit/Cargo.toml -- --nocapture
```

Run those commands from the repository root. Cargo selects tests by name
with the positional filter. Arguments after `--` go to the test harness.
Tests in a test binary can run concurrently, so give tests that write
files separate paths and avoid process-wide state where a dependency can
be passed in directly.

## Try it

Write a test that checks that an empty input yields zero for all three
counts in the capstone. Then choose a boundary case that would catch a
regression in line parsing, such as a valid final line with no newline.
Keep CLI-level assertions for behavior users and shell scripts observe.
[Original testing chapter](https://rust-book.cs.brown.edu/ch11-00-testing.html)

[Next — Choose how values are stored and shared](sharing.md)
