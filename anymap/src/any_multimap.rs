use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Debug, Default)]
struct Speed(u32);

#[derive(Debug, Default)]
struct Health(u32);

#[derive(Debug, Default)]
pub struct World {
    map: HashMap<TypeId, Vec<Box<dyn Any>>>,
}

impl World {

    pub fn register<T: Any>(&mut self) -> &mut Self {
        let id = TypeId::of::<T>();
        if self.map.contains_key(&id) {
            panic!("Component already registered");
        }
        self.map.insert(id, vec![]);
        self
    }

    pub fn new_entity(&mut self) {

    }

    pub fn with_component<T: Any>(&mut self, component: T) {
        let id = TypeId::of::<T>();
        self.map.entry(id).or_insert(Vec::new()).push(Box::new(component));
    }

    pub fn get_entity_component<T: Any>(&self, idx: usize) -> Option<&T> {
        let id = TypeId::of::<T>();
        let res = self.map.get(&id).map(|v| {
            v[idx].downcast_ref::<T>().unwrap()
        });
        return res;
    }

    pub fn get_components<T: Any>(&self) -> Option<&Vec<&T>> {
        let id = TypeId::of::<T>();
        match self.map.get(&id) {
            None => None,
            Some(vec) => {
                let cast = unsafe { std::mem::transmute::<&Vec<Box<dyn Any>>, &Vec<&T>>(vec) };
                Some(cast)
            }
        }
    }

    pub fn iter<T: Any>(&self) -> impl Iterator<Item=&'_ T> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut map = World::default();

        let st = Speed::default();
        let st2 = Health::default();

        map.with_component(st);
        map.with_component(st2);

        let a = map.iter::<Speed>();
        a.for_each(|e| {
            println!("{:?}", e)
        });


        let a = map.iter::<Health>();
        a.for_each(|e| {
            println!("{:?}", e)
        });
    }

    #[test]
    fn test_get() {
        let mut map = World::default();

        let st = Speed::default();
        let st2 = Health::default();

        map.with_component(st);
        map.with_component(st2);

        let option = map.get_components::<Health>();
        assert!(option.is_some());

        let x = option.unwrap();
        dbg!(x);
    }
}