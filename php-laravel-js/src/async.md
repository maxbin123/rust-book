# Use async when work spends time waiting

Rust and JavaScript share `async` and `await` syntax. They differ in when
the work starts. Calling a JS `async` function starts running its body up
to its first suspension point. Calling a Rust `async fn` evaluates the
arguments and returns a `Future`. The body starts when the future is polled.
Await it inside a running task or submit it to an executor.

## Await, join, or spawn

Awaiting `fetch_one().await` and then `fetch_two().await` runs those
future bodies in sequence. Joining both allows them to make progress
while the other waits. Spawning creates a separate task, which can
continue running while its handle is unused. An executor schedules tasks
and a runtime such as Tokio also provides timer and I/O support.

To run this standalone example, use the sandbox from the syntax chapter.
Add Tokio and place the Rust block in `src/main.rs`.

```sh
cargo +stable add tokio@1 --features macros,rt,time
cargo +stable run
```

```rust,ignore
use std::time::Duration;
use tokio::time::sleep;

async fn check_queue(name: &str, delay_ms: u64) -> &str {
    sleep(Duration::from_millis(delay_ms)).await;
    name
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (email, report) = tokio::join!(
        check_queue("email", 20),
        check_queue("report", 10),
    );
    println!("checked {email} and {report}");
}
```

`tokio::join!` polls both futures within the current task and returns both
outputs in argument order. The calls can overlap while waiting even though
this example uses one thread. `join!` waits for every branch even if one
returns `Err`. If you want an early `Err` from a group of fallible futures,
look at `tokio::try_join!`. Returning early drops the other inline futures.
Dropping a Tokio `JoinHandle` detaches its spawned task, so cancellation of
spawned work needs an explicit design.
[Tokio's `join!` documentation](https://docs.rs/tokio/latest/tokio/macro.join.html) ·
[Tokio's `JoinHandle` documentation](https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html)

## Keep async at the I/O boundary

Use async for a service that waits on network I/O or handles many waiting
operations. A plain `BufRead` loop is a good fit for the capstone's local
file. Long CPU-heavy loops and blocking calls can stall an executor thread.
Use dedicated threads or a bounded worker pool for sustained CPU work.
Use the runtime's blocking API when integrating a blocking call with an
async service.

A future can hold local state across an `.await`. If Tokio must move the
task between threads, values held across that point affect whether the
future is `Send`. Keep a synchronous lock guard out of the suspended
section by finishing the guarded operation before `.await`. A `Stream`
is like a JavaScript async iterator for a sequence that becomes ready
over time. `Pin` appears in polling APIs because some futures must stay
in place once pinned, including futures whose state borrows from itself.
For routine application code, `.await` and runtime APIs handle polling.
[Standard library `Future` trait](https://doc.rust-lang.org/std/future/trait.Future.html)

## Try it

Add a third queue check to the example. Make one branch return
`Result<&str, &'static str>` and compare `join!` and `try_join!` after
changing all branches to return that type. Then ask what should happen
when one check fails halfway through a real request.
[Original async chapter](https://rust-book.cs.brown.edu/ch17-00-async-await.html)

[Next — Audit a Laravel job export](job-audit.md)
