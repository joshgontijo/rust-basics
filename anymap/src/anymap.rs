use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct AnyMap {
    map: HashMap<TypeId, Box<dyn Any>>,
}

impl AnyMap {
    pub fn new() -> Self {
        AnyMap {
            map: Default::default()
        }
    }

    pub fn put<T: 'static>(&mut self, t: T) {
        let id = TypeId::of::<T>();
        self.map.entry(id).or_insert(Box::new(t));
    }

    pub fn get<T: 'static>(&self) -> Option<&'_ T> {
        let id = TypeId::of::<T>();
        let item = self.map.get(&id)?;
        item.downcast_ref::<T>()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct MyStruct1 {
        v: i32,
    }

    #[derive(Debug)]
    struct MyStruct2 {
        v: i32,
    }
    
    
    #[test]
    fn test() {

        let mut map = AnyMap { map: Default::default() };

        let st = MyStruct1 { v: 0 };
        let st2 = MyStruct2 { v: 0 };

        map.put(st);
        map.put(st2);

        let a = map.get::<MyStruct1>();
        println!("{:?}", a);

        let a = map.get::<MyStruct2>();
        println!("{:?}", a);

        let a = map.get::<String>();
        println!("{:?}", a);
    }
}