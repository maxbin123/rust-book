# Choose how values are stored and shared

JavaScript code can hand several parts of an application the same object and
let the garbage collector handle its reachability. Rust makes ownership and
shared mutation separate decisions. A value can often stay owned by one
struct and be passed around by reference. Reach for these types when that
shape doesn't fit the work.

| Type | When it fits |
| --- | --- |
| `Box<T>` | One owner needs heap allocation or indirection, as with a recursive type or owned trait object. |
| `Rc<T>` | Several owners need the same allocation on one thread. |
| `Arc<T>` | Several owners may use the same allocation across threads, subject to the inner type's thread-safety properties. |
| `RefCell<T>` | Single-threaded borrowing needs to be checked at runtime. |
| `Mutex<T>` | Access to shared data needs synchronization and one guard at a time. |
| `Weak<T>` | A link should refer to a reference-counted value without keeping it alive. |

## Indirection and cleanup

```rust
enum Rule {
    Allow,
    Invert(Box<Rule>),
}

fn accepts(rule: &Rule) -> bool {
    match rule {
        Rule::Allow => true,
        Rule::Invert(inner) => !accepts(inner),
    }
}

let rule = Rule::Invert(Box::new(Rule::Allow));
assert!(!accepts(&rule));
```

The recursive field has a known size because the `Box` holds the nested
rule indirectly. `Deref` lets an appropriate pointer-like type act like
a reference to its target as in the `accepts(inner)` call. `Drop` provides
cleanup when a value is dropped. In ordinary scope exit, owned values such
as files and lock guards release their resources as they are dropped.

`Rc::clone(&handle)` increments a reference count and keeps the same
allocation. `Arc::clone` does the same with an atomic count. `Arc` alone
doesn't provide synchronized mutable access to the contents. Combine
`Arc` with `Mutex` when several threads really need to update the same
state. With `RefCell`, a conflicting borrow panics with `borrow` or
`borrow_mut`. `try_borrow` and `try_borrow_mut` report the conflict as an
error instead.

```rust
use std::rc::Rc;

let config = Rc::new(String::from("email"));
let observer = Rc::downgrade(&config);
assert!(observer.upgrade().is_some());
drop(config);
assert!(observer.upgrade().is_none());
```

Reference counting frees a value when its last strong owner is dropped.
Strong references arranged in a cycle can keep one another alive. For a
tree, a parent can own children through strong references and children
can refer back through `Weak`.

## Try it

Make two `Rc` handles for the same settings. Drop the first and confirm
the weak observer can still upgrade. Drop the second and check that it now
returns `None`. [Original smart pointers chapter](https://rust-book.cs.brown.edu/ch15-00-smart-pointers.html)

[Next — Run work on threads](threads.md)
