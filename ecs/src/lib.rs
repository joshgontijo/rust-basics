#![feature(type_alias_impl_trait)]
#![feature(associated_type_defaults)]
#![feature(explicit_generic_args_with_impl_trait)]

extern crate core;

use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::component::{Component, Components, ComponentsIter, Fetch};
use crate::entity_builder::{EntityBuilder, EntityId};
use crate::resource::Resources;
use crate::system::{Sys, Systems};

mod entity_builder;
pub mod system;
pub mod component;
mod resource;


#[derive(Default)]
pub struct World<C> {
    resources: Resources,
    components: Components,
    systems: Systems<C>,
}


#[derive(Default)]
pub struct WorldBuilder<C> {
    components: HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any>>>>>,
    _m: PhantomData<C>,
}

impl<C> WorldBuilder<C> {
    pub fn register_component<T: Any>(mut self) -> Self {
        let id = TypeId::of::<T>();
        if self.components.contains_key(&id) {
            panic!("Component already registered");
        }
        self.components.insert(id, vec![]);
        WorldBuilder {
            components: self.components,
            _m: PhantomData,
        }
    }

    pub fn build(self) -> World<C> {
        World {
            components: Components::new(self.components),
            resources: Resources::default(),
            systems: Systems { items: Default::default() },
        }
    }
}

pub fn builder<C>() -> WorldBuilder<C> {
    WorldBuilder {
        components: Default::default(),
        _m: PhantomData,
    }
}

impl<C: 'static> World<C> {
    pub fn add_resource<T: Any>(&mut self, resource: T) -> &mut Self {
        self.resources.add_resource(resource);
        self
    }

    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_resource()
    }

    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_resource_mut()
    }

    pub fn remove_resource<T: Any>(&mut self) -> Option<T> {
        self.resources.remove_resource()
    }

    pub fn new_entity(&mut self) -> EntityBuilder {
        let entity_id = self.components.new_entity();
        return EntityBuilder {
            id: entity_id,
            components: &mut self.components,
        };
    }


    pub fn get_component<T: Any>(&self, entity_id: EntityId) -> Option<Component<T>> {
        self.components.get_component(entity_id)
    }

    pub fn query<Tuple>(&self) -> ComponentsIter<Tuple> {
        self.components.query::<Tuple>()
    }

    pub fn with_system<T>(&mut self, f: impl Sys<Query=T, Ctx=C> + 'static) -> &mut Self
        where
            T: Fetch + 'static,
            C: 'static
    {
        self.systems.add_system::<T>(Box::new(f));
        self
    }

    pub fn run_systems(&mut self, ctx: &mut C) {
        let components = &self.components;
        for system in self.systems.iter_mut() {
            system.run(ctx, components)
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::component::Component;

    use super::*;

    #[derive(Debug, Default, Eq, PartialEq)]
    struct Speed(u32);

    #[derive(Debug, Default, Eq, PartialEq)]
    struct Health(u32);

    #[derive(Debug)]
    struct Ctx;

    #[test]
    fn iter() {
        let mut world = builder::<Ctx>()
            .register_component::<Health>()
            .register_component::<Speed>()
            .build();

        world.new_entity()
            .with_component(Health(100))
            .with_component(Speed(1));

        world.new_entity()
            .with_component(Health(100));


        let a = world.query::<(Speed, Health)>();
        a.for_each(|(e)| {
            println!("{:?}", e)
        });


        let a = world.query::<(Health, )>();
        a.for_each(|e| {
            println!("{:?}", e)
        });
    }

    #[test]
    fn query() {
        let mut world = builder::<Ctx>()
            .register_component::<Health>()
            .register_component::<Speed>()
            .build();

        world.new_entity()
            .with_component(Health(100))
            .with_component(Speed(1));

        world.new_entity()
            .with_component(Speed(1));


        let mut query = world.query::<(Speed, Health)>();

        while let Some((speed, health)) = query.next() {
            speed.as_ref_mut().0 += 1
        }
    }

    #[test]
    fn get_components() {
        let mut world = builder::<Ctx>()
            .register_component::<Health>()
            .register_component::<Speed>()
            .build();

        world.new_entity()
            .with_component(Health(100))
            .with_component(Speed(1));


        let component = world.get_component::<Speed>(0);
        assert!(component.is_some());
        assert_eq!(component.unwrap().as_ref().0, 1);
    }

    #[test]
    fn test_resource() {
        struct WorldWidth(u32);

        let mut world = builder::<Ctx>().build();
        world.add_resource(WorldWidth(123));

        let v = world.get_resource::<WorldWidth>().unwrap().0;
        assert_eq!(v, 123);

        world.get_resource_mut::<WorldWidth>().unwrap().0 += 1;
        let v = world.get_resource::<WorldWidth>().unwrap().0;
        assert_eq!(v, 124);


        world.get_resource_mut::<WorldWidth>().unwrap().0 += 1;
        let v = world.remove_resource::<WorldWidth>();
        assert!(v.is_some());

        let found = world.get_resource::<WorldWidth>();
        assert!(found.is_none());
    }


    struct MovementSystem;

    impl Sys for MovementSystem {
        type Query = (Speed, Health);
        type Ctx = Ctx;

        fn run(&self, ctx: &mut Self::Ctx, (speed, health): <Self::Query as Fetch>::Data) {
            println!("{speed:?} - {health:?}")
        }
    }


    #[test]
    fn test_run_system() {
        let mut world = builder::<Ctx>()
            .register_component::<Health>()
            .register_component::<Speed>()
            .build();

        world.new_entity()
            .with_component(Health(100))
            .with_component(Speed(1));

        let mut ctx = Ctx;

        world.with_system::<(Speed, Health)>(MovementSystem);

        for i in 0..10 {
            do_something(&mut ctx);
            world.run_systems(&mut ctx);
        }
    }

    fn do_something(ctx: &mut Ctx) {
        println!("{ctx:?}");
    }
}