#![feature(type_alias_impl_trait)]

extern crate core;

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::iter;
use std::path::Iter;

use crate::entity_builder::{EntityBuilder, EntityId};

mod entity_builder;

#[derive(Debug, Default, Eq, PartialEq)]
struct Speed(u32);

#[derive(Debug, Default, Eq, PartialEq)]
struct Health(u32);

#[derive(Debug, Default)]
pub struct World {
    entity_count: usize,
    components: HashMap<TypeId, Vec<Option<Box<dyn Any>>>>,
    resources: HashMap<TypeId, Box<dyn Any>>,
}


#[derive(Default)]
pub struct WorldBuilder {
    components: HashMap<TypeId, Vec<Option<Box<dyn Any>>>>,
}

impl WorldBuilder {
    pub fn register_component<T: Any>(mut self) -> Self {
        let id = TypeId::of::<T>();
        if self.components.contains_key(&id) {
            panic!("Component already registered");
        }
        self.components.insert(id, vec![]);
        WorldBuilder {
            components: self.components
        }
    }

    pub fn build(self) -> World {
        World {
            entity_count: 0,
            components: self.components,
            resources: Default::default(),
        }
    }
}

impl World {
    pub fn builder() -> WorldBuilder {
        Default::default()
    }

    pub fn add_resource<T: Any>(&mut self, resource: T) -> &mut Self {
        self.resources.insert(TypeId::of::<T>(), Box::new(resource));
        self
    }

    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get(&TypeId::of::<T>()).map(|v| {
            v.downcast_ref::<T>().unwrap()
        })
    }

    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_mut(&TypeId::of::<T>()).map(|v| {
            v.downcast_mut::<T>().unwrap()
        })
    }

    pub fn remove_resource<T: Any>(&mut self) -> Option<T> {
        self.resources.remove(&TypeId::of::<T>())
            .map(|v| {
                *v.downcast::<T>().unwrap()
            })
    }

    pub fn new_entity(&mut self) -> EntityBuilder {
        self.components.values_mut().for_each(|components| components.push(None));

        let entity_id = self.entity_count;
        self.entity_count += 1;
        return EntityBuilder {
            id: entity_id,
            components: &mut self.components,
        };
    }

    fn get_components<T: Any>(&self) -> impl Iterator<Item=&'_ T> {
        let components = self.components.get(&TypeId::of::<T>()).expect("Component not registered");
        components.iter().filter_map(|e| {
            match e {
                None => None,
                Some(c) => Some(c.downcast_ref::<T>().unwrap())
            }
        })
    }

    pub fn get_entity_component<T: Any>(&self, entity_id: EntityId) -> Option<&T> {
        let id = TypeId::of::<T>();
        let components = self.components.get(&id).expect("Component not registered");

        components.get(entity_id).map(|e| {
            match e {
                None => None,
                Some(c) => Some(c.downcast_ref::<T>().unwrap())
            }
        }).flatten()
    }

    pub fn iter<T: Any>(&self) -> impl Iterator<Item=&'_ T> {
        let id = TypeId::of::<T>();
        let item = self.components.get(&id).expect("Component no registered");

        item.iter().filter_map(|e| {
            match e {
                None => None,
                Some(v) => {
                    v.downcast_ref::<T>()
                }
            }
        })
    }
}

trait System {
    type Data;
    fn run(&self, data: Self::Data);
}

struct TestSystem;

impl System for TestSystem {
    type Data = (Speed, Health);

    fn run(&self, data: Self::Data) {}
}

fn test_system() {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter() {
        let mut world = World::builder()
            .register_component::<Health>()
            .register_component::<Speed>()
            .build();

        world.new_entity()
            .with_component(Health(100))
            .with_component(Speed(1));

        world.new_entity()
            .with_component(Health(100));


        let a = world.iter::<Speed>();
        a.for_each(|e| {
            println!("{:?}", e)
        });


        let a = world.iter::<Health>();
        a.for_each(|e| {
            println!("{:?}", e)
        });
    }

    #[test]
    fn query() {
        let mut world = World::builder()
            .register_component::<Health>()
            .register_component::<Speed>()
            .build();

        world.new_entity()
            .with_component(Health(100))
            .with_component(Speed(1));

        world.new_entity()
            .with_component(Speed(1));
    }

    #[test]
    fn get_components() {
        let mut world = World::builder()
            .register_component::<Health>()
            .register_component::<Speed>()
            .build();

        world.new_entity()
            .with_component(Health(100))
            .with_component(Speed(1));

        world.new_entity()
            .with_component(Speed(2));


        let mut iter = world.get_components::<Speed>();

        assert_eq!(Some(&Speed(1)), iter.next());
        assert_eq!(Some(&Speed(2)), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn system() {
        fn test_system(query: (Health, Speed)) {
            let (health, speed) = query;
            println!("{health:?} {speed:?}");
        }

        let mut world = World::builder()
            .register_component::<Health>()
            .register_component::<Speed>()
            .build();

        world.new_entity()
            .with_component(Health(100))
            .with_component(Speed(1))
            .with_system(test_system);


        let mut iter = world.get_components::<Speed>();

        assert_eq!(Some(&Speed(1)), iter.next());
        assert_eq!(Some(&Speed(2)), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_resource() {
        struct WorldWidth(u32);

        let mut world = World::builder().build();
        world.add_resource(WorldWidth(123));

        let v = world.get_resource::<WorldWidth>().unwrap().0;
        assert_eq!(v, 123);

        world.get_resource_mut::<WorldWidth>().unwrap().0 += 1;
        let v = world.get_resource::<WorldWidth>().unwrap().0;
        assert_eq!(v, 124);


        world.get_resource_mut::<WorldWidth>().unwrap().0 += 1;
        let v = world.remove_resource::<WorldWidth>();
        assert!(v.is_some())
    }
}