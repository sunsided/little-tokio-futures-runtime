# A trivial futures/`async` runtime in Rust

This is a playground project to better understand the internals
of how futures work in Rust.

```shell
cargo run --example tasks
```

The file [`src/lib.rs`](src/lib.rs) contains the runtime and task
definitions, while [`examples/tasks.rs`](examples/tasks.rs) contains
a test application.
