#![feature(test)]
#![feature(ptr_metadata)]
extern crate anymap;
extern crate test;

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::iter::Map;
use std::mem;
use std::ptr::DynMetadata;
use std::slice::Iter;
use test::bench::iter;

use anymap::AnyMap;

#[derive(Debug)]
struct MyStruct {
    v: i32,
}

impl MyStruct {
    fn add(&mut self, val: i32) {
        self.v += val;
    }
}

impl Handler<String> for MyStruct {
    fn on_event(&mut self, ev: &String) {
        println!("{:?}", ev)
    }
}

impl Handler<u32> for MyStruct {
    fn on_event(&mut self, ev: &u32) {
        println!("{:?}", ev)
    }
}

fn main() {
    // let mut map = AnyMultiMap { map: Default::default() };
    //
    // let st = MyStruct { v: 0 };
    // let st2 = MyStruct { v: 0 };
    //
    // map.put(st);
    // map.put(st2);
    //
    // let a = map.get::<MyStruct>();
    // a.for_each(|e| {
    //     println!("{:?}", e)
    // })

    let st = MyStruct { v: 0 };
    let mut bus = EventBus { table: Default::default() };
    bus.register2(MyStruct::add);
}

///////////////////////

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
//////////////////////

trait Handler<T> {
    fn on_event(&mut self, ev: &T);
}

struct EventBus {
    table: HashMap<TypeId, AnyMultiMap>,
}

impl EventBus {
    fn register<H, T: 'static>(&mut self, id: &str, handler: H) where H: Handler<T> + 'static {
        let ev_type = TypeId::of::<T>();
        self.table.entry(ev_type)
            .or_insert(AnyMultiMap::new())
            .put(handler);
    }

    fn register2<F, T, E>(&mut self, f: F)
        where F: FnMut(T, E) + 'static,
              E: 'static
    {
        let ev_type = TypeId::of::<E>();
        self.table.entry(ev_type)
            .or_insert(AnyMultiMap::new())
            .put(f);
    }

    fn send<T>(&self, ev: &T) {
        // let ev_type = TypeId::of::<T>();
        // if let Some(map) = self.table.get(&ev_type) {
        //     let mut handlers = map.get::<T>();
        //     let option: Option<&dyn Handler<T>> = handlers.next();
        //     while let Some(handler) = handlers.next() {
        //         handler(ev);
        //     }
        // }
    }
}
