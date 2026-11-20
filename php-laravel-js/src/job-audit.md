# Capstone — Audit a Laravel job export

Build a small CLI that validates an exported job manifest before you use
it for reporting. Think of it as the file-processing part of an Artisan
command. It has one input, a result a shell script can read, and an exit
status that tells the caller whether processing succeeded.

## Define the file contract

Each nonblank line is UTF-8 text with `kind|id|payload`. The allowed
kinds are `email` and `report`. The ID fits in a `u64` and must be positive.
Fields are trimmed. The payload is nonempty text on the same line and can
contain `|` because only the first two separators split fields. Blank
lines are skipped and still count toward line numbers. The audit stops
at the first malformed record or read error.

```text
email|42|order shipped
report|99|weekly metrics
email|105|receipt ready
```

Run the reference implementation from the repository root.

```sh
cargo +stable run --manifest-path php-laravel-js/job-audit/Cargo.toml -- php-laravel-js/job-audit/sample.jobs
```

```text
jobs=3 email=2 report=1
```

The CLI uses exit code `0` when it prints a summary, `1` for processing
failures, and `2` for incorrect usage. For an invalid record, stderr includes
the filename and physical line number. Stdout stays empty on a failed audit.

## Write the parser first

Design `parse_job(line: &str) -> Result<Job<'_>, ParseJobError>`. Split
the line with `splitn(3, '|')` and destructure the three `Option<&str>`
values with `let ... else`. Match the kind, parse the ID as `NonZeroU64`,
and reject an empty payload. Store the kind and ID by value and keep
the payload borrowed from the line. Test a valid record, a zero ID, one
missing field, and a payload containing `|`.

## Reuse the file buffer

Design `audit(input: impl BufRead) -> Result<Summary, AuditError>`.
Use `read_line` into a `String` that you clear between iterations.
The `Job<'_>` exists during the count update and the payload borrow ends
before the next read. Cloning the payload would be useful if the audit
stored jobs beyond the current iteration. The report only needs counters.
Annotate both read and parse failures with the current line number.

Use an in-memory byte slice when testing the audit. Check valid counts,
blank lines, a final line without a newline, and the line number of
a malformed record. In `main.rs`, accept a filesystem path, open it,
and call `audit`. Report errors to stderr and print the summary on success.

## Extend it

Add a `cleanup` job kind and an accompanying counter. Follow the compiler
through each exhaustive `match`, then add a file-level test. For a second
exercise, change the format to newline-delimited JSON. Use a typed
deserialization step at the input boundary and keep the rest of the audit
working with validated values.

[Reference parser, audit, and tests](https://github.com/maxbin123/rust-book/blob/main/php-laravel-js/job-audit/src/lib.rs) ·
[Reference CLI](https://github.com/maxbin123/rust-book/blob/main/php-laravel-js/job-audit/src/main.rs) ·
[Original I/O project](https://rust-book.cs.brown.edu/ch12-00-an-io-project.html) ·
[Original threaded server project](https://rust-book.cs.brown.edu/ch21-00-final-project-a-web-server.html)

[Next — Advanced Rust and further reading](further-reading.md)
