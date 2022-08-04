use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::sync::Mutex;

fn main() {
    let pool = Pool::new(1, || Vec::<u32>::new());

    println!("FIRST");
    {
        let mut pooled = pool.pull().unwrap();
        pooled.push(1);
    }

    println!("SECOND");
    {
        let mut pooled = pool.pull().unwrap();
        pooled.push(1);
    }
}

struct Pool<T> {
    objects: Mutex<Vec<T>>,
}

impl<T> Pool<T> {
    pub fn new<F>(capacity: usize, init: F) -> Self
        where F: Fn() -> T
    {
        let mut items = Vec::new();

        for _ in 0..capacity {
            items.push(init());
        }

        Self {
            objects: Mutex::new(items)
        }
    }

    pub fn attach(&self, t: T) {
        self.objects.lock()
            .unwrap()
            .push(t);
    }

    pub fn pull(&self) -> Option<Pooled<T>> {
        println!("Pulling from pool");
        self.objects.lock()
            .ok()?
            .pop()
            .map(|data| Pooled::new(self, data))
    }
}


struct Pooled<'a, T> {
    pool: &'a Pool<T>,
    data: ManuallyDrop<T>,
}


impl<'a, T> Pooled<'a, T> {
    pub fn new(pool: &'a Pool<T>, t: T) -> Self {
        Self {
            pool,
            data: ManuallyDrop::new(t),
        }
    }
}

impl<'a, T> Deref for Pooled<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, T> DerefMut for Pooled<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'a, T> Drop for Pooled<'a, T> {
    fn drop(&mut self) {
        println!("Recycling...");
        unsafe {
            let t = ManuallyDrop::take(&mut self.data);
            self.pool.attach(t)
        }
    }
}

