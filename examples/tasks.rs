use std::time::{Duration, Instant};
use futures_runtime::delay::Delay;
use futures_runtime::Runtime;

async fn hello_world() {
    println!("Hello, world!");
}

async fn delayed_message() {
    println!("Starting delay ...");
    let before = Instant::now();
    Delay::new(Duration::from_secs(2)).await;
    println!("Delayed message for {:1.2} seconds", (Instant::now() - before).as_secs_f32());
}

fn main() {
    let rt = Runtime::new();

    rt.spawn(hello_world());
    rt.spawn(delayed_message());

    rt.run();
}
