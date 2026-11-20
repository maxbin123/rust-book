# Use traits, generics, and lifetimes to design APIs

A Rust trait can describe a boundary much like a PHP interface for services
bound in Laravel's container. Suppose your job tool can hand an audit entry
to different destinations. The trait describes the operation. The signature
decides how to select the implementation.

```rust
trait AuditSink {
    fn record(&mut self, kind: &str);
}

#[derive(Default)]
struct MemorySink(Vec<String>);

impl AuditSink for MemorySink {
    fn record(&mut self, kind: &str) {
        self.0.push(kind.to_owned());
    }
}

fn audit_one<S: AuditSink>(sink: &mut S, kind: &str) {
    sink.record(kind);
}

fn audit_selected(sink: &mut dyn AuditSink, kind: &str) {
    sink.record(kind);
}

let mut sink = MemorySink::default();
audit_one(&mut sink, "email");
audit_selected(&mut sink, "report");
assert_eq!(sink.0, ["email", "report"]);
```

`S: AuditSink` is a bound. The compiler specializes generic code for each
concrete type used and can resolve the method statically.
`&mut impl AuditSink` is another way to write the single-argument generic
signature. `&mut dyn AuditSink` dispatches through a vtable at runtime and
borrows an existing implementation. `Box<dyn AuditSink>` owns a boxed
implementation and works when selecting between concrete implementations
at runtime. If all alternatives are known and belong to the domain, an enum
with a method can also express the choice.

`-> impl AuditSink` hides a concrete type while promising that it implements
the trait. A function with that return type must pick the same underlying
type for all return paths. Some traits have methods that cannot be called
through a trait object. For example, an ordinary generic method requires
choosing type arguments at compile time.

Prefer composing struct fields and implementing traits over looking for a
base class hierarchy. For Laravel-style dependency injection, you can pass
an instance into a constructor and store it as a generic field or trait
object. The main function can assemble the production implementations and a
test can assemble its own.

## Use lifetimes to relate a borrow to its source

```rust
fn before<'a>(input: &'a str, separator: &str) -> &'a str {
    input.split_once(separator).map_or(input, |(head, _)| head)
}

let line = String::from("email|42|order shipped");
let kind = {
    let separator = String::from("|");
    before(&line, &separator)
};

assert_eq!(kind, "email");
```

The `'a` on `input` and the output means the returned view is valid within
the borrow of `input`. The separator has its own lifetime and can go out
of scope while the result remains usable. Lifetime annotations describe a
relationship the compiler checks. They cannot extend the life of a value.
With one borrowed input, the compiler can normally infer this relationship.
Multiple input references can make an explicit annotation necessary.

```compile_fail
fn invalid<'a>() -> &'a str {
    let scratch = String::from("email");
    scratch.as_str()
}
```

`scratch` is dropped when the function returns. Return `String` if the
caller should own that generated value. For a type such as `Job<'a>`, the
same principle applies to a stored field like `payload: &'a str`. The job
stays usable while the borrowed source text stays valid.

## Try it

Add a second `AuditSink` implementation that counts entries. Accept a
`Vec<Box<dyn AuditSink>>` in a sandbox and record one entry in each sink.
Then change `before` to return both segments from `split_once` and annotate
the type so both borrow the input. [Original traits and lifetimes chapter](https://rust-book.cs.brown.edu/ch10-00-generics.html) ·
[Original trait objects chapter](https://rust-book.cs.brown.edu/ch18-02-trait-objects.html)

[Next — Test the contract](tests.md)
