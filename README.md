# Little Tokio: A trivial futures/`async` runtime in Rust

This is a toy futures executor / `async` runtime built to better understand the internals
of how futures work in Rust. The name, of course, being inspired by [tokio](https://github.com/tokio-rs/tokio).

TL;DR:

```shell
cargo run --example tasks
```

The file [`src/lib.rs`](src/lib.rs) contains the runtime and task
definitions, while [`examples/tasks.rs`](examples/tasks.rs) contains
a test application.
[`src/delay.rs`](src/delay.rs) contains an implementation for `Delay`,
a make-shift `await`able `delay` function.
