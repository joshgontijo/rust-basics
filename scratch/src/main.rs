use std::any::{Any, TypeId};
use std::collections::HashMap;

fn main() {
    let storage = Storage::new();



}

struct Storage<C> {
    map: HashMap<TypeId, fn(&mut C, dyn Any)>,
}

impl<C> Storage<C>  {
    pub fn new() -> Self {
        Self { map: Default::default() }
    }

    pub fn add<T>(&mut self, f: fn(&mut C, T)) where T: Any {
        self.map.insert(TypeId::of::<T>(), );
    }

}
