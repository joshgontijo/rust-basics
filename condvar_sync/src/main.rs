use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::collections::vec_deque::VecDeque;
use std::fmt::Debug;
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Condvar, mpsc, Mutex};
use std::sync::atomic::Ordering::AcqRel;
use std::task::{Context, Poll, Waker};
use std::thread;
use std::thread::{Thread, ThreadId};
use std::time::Duration;

fn print<T: Debug>(t: T) {
    println!("{:?}", t);
}


fn main() {
    let (sender, recv) = completable_future();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(2000));
        println!("RESUMING");
        sender.complete("YOLO".to_string());
    });

    println!("AWAITING");
    let x = recv.join();
    println!("RESULT: {:?}", x);
}

fn completable_future<T>() -> (Sender<T>, Receiver<T>) {
    let arc = Arc::new((Mutex::default(), Condvar::default()));

    let sender = Sender {
        inner: arc.clone()
    };

    let recv = Receiver {
        inner: arc.clone()
    };

    return (sender, recv);
}

type Shared<T> = (Mutex<Option<T>>, Condvar);

struct Sender<T> {
    inner: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    pub fn complete(self, value: T) {
        let mut guard = self.inner.0.lock().unwrap();
        *guard = Some(value);
        drop(guard);
        self.inner.1.notify_all();
    }
}

struct Receiver<T> {
    inner: Arc<Shared<T>>,
}


impl<T> Receiver<T> {
    pub fn join(self) -> T {
        loop {
            let mut guard = self.inner.0.lock().unwrap();
            let mut cvar = &self.inner.1;
            let mut val = guard.take();
            while val.is_none() {
                guard = cvar.wait(guard).unwrap();
                val = guard.take();
            }

            return val.unwrap();
        }
    }
}