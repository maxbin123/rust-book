# Model jobs with structs, enums, and patterns

A Laravel job export or a JSON message begins as data you have to validate.
Make that validation boundary explicit. In PHP you might construct a DTO
after checking an associative array. In JavaScript you might validate a
payload with a schema and switch on its `kind` property. In Rust a struct
holds fields present on every job, and an enum puts the fields belonging to
each variant in their own place.

```rust
use std::num::NonZeroU64;

enum JobKind {
    Email { recipient: String },
    Report { month: u8 },
}

struct Job {
    id: NonZeroU64,
    kind: JobKind,
}

impl Job {
    fn label(&self) -> &'static str {
        match &self.kind {
            JobKind::Email { .. } => "email",
            JobKind::Report { .. } => "report",
        }
    }

    fn recipient(&self) -> Option<&str> {
        match &self.kind {
            JobKind::Email { recipient } => Some(recipient.as_str()),
            JobKind::Report { .. } => None,
        }
    }
}

let job = Job {
    id: NonZeroU64::new(42).expect("42 is nonzero"),
    kind: JobKind::Email { recipient: "ops@example.com".into() },
};

assert_eq!(job.label(), "email");
assert_eq!(job.recipient(), Some("ops@example.com"));
```

The type makes it impossible to construct a `Report` with a recipient field.
It also makes zero invalid as a job ID. A month still needs validation
because `u8` includes values outside `1..=12`. A private field and a checked
constructor can enforce the narrower rule if it matters across the program.

Methods belong in an `impl` block. `&self` borrows the instance, `&mut self`
borrows it exclusively, and `self` takes it by value. An associated function
such as `NonZeroU64::new` has no `self` receiver. In `recipient`, matching
`&self.kind` lets the method borrow the recipient from inside the job. The
`'static` labels in `label` are string literals available for the life of the
program.

## Pick a pattern for the job

`match` covers all possibilities and can produce a value. For a domain enum,
listing each variant forces you to decide what to do when you later add one.
Use `if let` for a branch you care about where you can skip the other cases.
Use `let ... else` to pull out the expected case and leave early otherwise.

```rust
fn canonical_queue(value: Option<&str>) -> &str {
    let Some(value) = value else {
        return "default";
    };

    match value.trim() {
        "" => "default",
        "mail" | "emails" => "email",
        queue => queue,
    }
}

assert_eq!(canonical_queue(Some(" mail ")), "email");
assert_eq!(canonical_queue(None), "default");
```

Pattern matching can destructure tuples, structs, and enum variants. `_`
ignores a value and `..` skips remaining fields. A guard such as
`Some(priority) if priority > 10` adds a condition and still needs coverage
for the cases where the condition fails. An ordinary `let` destructuring
pattern must always match. `let ... else` and `if let` handle patterns that
can fail.

## Try it

Add `Cleanup { keep_days: u16 }` to `JobKind`. Update both methods without a
catch-all and notice which decisions the compiler makes you revisit.
[Original enums and matching chapter](https://rust-book.cs.brown.edu/ch06-00-enums.html) ·
[Original patterns chapter](https://rust-book.cs.brown.edu/ch19-00-patterns.html)

[Next — Make errors part of the contract](errors.md)
