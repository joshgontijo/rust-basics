extern crate core;

use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::entity_builder::{EntityBuilder};
use crate::query::QueryBuilder;

mod entity_builder;
mod query;

#[derive(Debug, Default)]
struct Speed(u32);

#[derive(Debug, Default)]
struct Health(u32);

#[derive(Debug, Default)]
pub struct World {
    entity_count: usize,
    map: HashMap<TypeId, Vec<Option<Box<dyn Any>>>>,
}

pub struct WorldBuilder {
    map: HashMap<TypeId, Vec<Option<Box<dyn Any>>>>,
}

impl WorldBuilder {
    pub fn register_component<T: Any>(mut self) -> Self {
        let id = TypeId::of::<T>();
        if self.map.contains_key(&id) {
            panic!("Component already registered");
        }
        self.map.insert(id, vec![]);
        WorldBuilder {
            map: self.map
        }
    }

    pub fn build(self) -> World {
        World {
            entity_count: 0,
            map: self.map,
        }
    }
}

impl World {
    pub fn builder() -> WorldBuilder {
        WorldBuilder {
            map: Default::default()
        }
    }

    pub fn new_entity(&mut self) -> EntityBuilder {
        self.map.values_mut().for_each(|components| components.push(None));

        let entity_id = self.entity_count;
        self.entity_count += 1;
        return EntityBuilder {
            id: entity_id,
            components: &mut self.map,
        };
    }

    pub fn query(&self) -> QueryBuilder {
        QueryBuilder::default()
    }

    pub fn get_entity_component<T: Any>(&self, idx: usize) -> Option<&T> {
        let id = TypeId::of::<T>();
        let components = self.map.get(&id).expect("Component not registered");

        components.get(idx).map(|e| {
            match e {
                None => None,
                Some(c) => Some(c.downcast_ref::<T>().unwrap())
            }
        }).flatten()
    }

    pub fn iter<T: Any>(&self) -> impl Iterator<Item=&'_ T> {
        let id = TypeId::of::<T>();
        let item = self.map.get(&id).expect("Component no registered");

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


        let query = world.query()
            .with_component::<Speed>()
            .run();



    }
}