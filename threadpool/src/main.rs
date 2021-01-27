use futures::executor::{ThreadPool, block_on};
use futures::task::SpawnExt;
use std::time::Duration;
use std::thread::sleep;

fn main() -> Result<(), std::io::Error> {
    let pool = ThreadPool::builder()
        .name_prefix("my-pool-")
        .create()?;

    let result = pool.spawn_with_handle(do_async()).unwrap();
    let computed = block_on(result);

    println!("RESULT: {}", computed);
    Ok(())
}

async fn do_async() -> i32 {
    sleep(Duration::from_secs(5));
    if let Some(name) = std::thread::current().name() { println!("DONE {}", name) }
    1
}

