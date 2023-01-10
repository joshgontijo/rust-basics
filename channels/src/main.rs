use std::sync::{mpsc, Mutex, Arc};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::sync::mpsc::Receiver;
use std::fmt::Debug;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    let rx_mut = Arc::new(Mutex::new(rx));

    let a = Arc::clone(&rx_mut);
    let b = Arc::clone(&rx_mut);
    let handle1 = thread::spawn(move || do_work(a));
    let handle2 = thread::spawn(move || do_work(b));

    tx.send("ABC".to_owned()).unwrap();
    tx.send("DEF".to_owned()).unwrap();

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn do_work<T>(rx: Arc<Mutex<Receiver<T>>>) where T: Debug {
    println!("Waiting");
    sleep(Duration::from_secs(4));
    println!("Done waiting");

    if let Ok(a) = rx.lock().unwrap().recv() { println!("RECEIVED: {:?}", a) }
}
