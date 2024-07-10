use futures_runtime::Runtime;
use futures_time::task::sleep;
use futures_time::time::Duration;

async fn hello_world() {
    println!("Hello, world!");
}

async fn delayed_message() {
    println!("Starting delay...");
    sleep(Duration::from_secs(2)).await;
    println!("Delayed message!");
}

fn main() {
    let rt = Runtime::new();

    rt.spawn(hello_world());
    rt.spawn(delayed_message());

    rt.run();
}
