use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Debug)]
struct MyStruct1 {
    v: i32,
}

#[derive(Debug)]
struct MyStruct2 {
    v: i32,
}

fn main() {
    let mut map = AnyMultiMap { map: Default::default() };

    let st = MyStruct1 { v: 0 };
    let st2 = MyStruct2 { v: 0 };

    map.put(st);
    map.put(st2);

    let a = map.get::<MyStruct1>();
    a.for_each(|e| {
        println!("{:?}", e)
    });


    let a = map.get::<MyStruct2>();
    a.for_each(|e| {
        println!("{:?}", e)
    });
}


pub struct AnyMultiMap {
    map: HashMap<TypeId, Vec<Box<dyn Any>>>,
}

impl AnyMultiMap {
    pub fn new() -> Self {
        AnyMultiMap {
            map: Default::default()
        }
    }

    pub fn put<T: 'static>(&mut self, t: T) {
        let id = TypeId::of::<T>();
        self.map.entry(id).or_insert(Vec::new()).push(Box::new(t));
    }

    pub fn get<T: 'static>(&self) -> impl Iterator<Item=&'_ T> {
        let id = TypeId::of::<T>();
        let item = self.map.get(&id);

        let iter_a = if let Some(v) = item {
            Some(v.iter().map(|e| e.downcast_ref::<T>().unwrap()))
        } else {
            None
        };

        std::iter::empty().chain(iter_a.into_iter().flatten())
    }
}