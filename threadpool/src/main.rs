use futures::executor::{ThreadPool, block_on};
use futures::task::SpawnExt;
use std::time::Duration;
use std::thread::sleep;
use futures::TryFutureExt;
use std::sync::mpsc;
use std::sync::atomic::{AtomicIsize, AtomicUsize, AtomicU32};

fn main() -> Result<(), std::io::Error> {
    let pool = ThreadPool::builder()
        .name_prefix("my-pool-")
        .create()?;

    let future = do_async().and_then(add10);
    let result = pool.spawn_with_handle(future).unwrap();
    let computed = block_on(result);

    println!("RESULT: {:?}", computed);
    Ok(())
}

/**
Alternatively:
let future = async {
    ....
}
**/
async fn do_async() -> Result<i32, ()> {
    sleep(Duration::from_secs(5));
    if let Some(name) = std::thread::current().name() { println!("do_async {}", name) }
    Ok(1)
}

async fn add10(x: i32) -> Result<i32, ()> {
    sleep(Duration::from_secs(5));
    if let Some(name) = std::thread::current().name() { println!("add10 {}", name) }
    Ok(x + 10)
}

