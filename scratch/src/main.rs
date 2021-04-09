use serde::{Deserialize, Serialize};
use std::borrow::{Cow, Borrow, BorrowMut};
use std::cell::{Cell, RefCell};
use std::sync::Arc;
use std::fs::File;
use std::thread;

fn main() {
    let mut data = Arc::new(vec![Arc::new(User::of(0))]);

    let copy1 = Arc::clone(&data);
    let t1 = thread::spawn(move || {
        println!("{:?}", copy1);
        return 1;
    });

    let copy2 = Arc::clone(&data);
    let t2 = thread::spawn(move || {
        let mut vec1 = copy2[..].to_vec();
        vec1.push(Arc::new(User::of(1)));
        vec1.push(Arc::new(User::of(2)));
        return Arc::new(vec1);
    });


    t1.join().unwrap();
    data = t2.join().unwrap();
    println!("{:?}", data);
}

#[derive(Debug, Default)]
struct User(u32);

impl User {
    fn of(i: u32) -> Self { User { 0: i } }
}
