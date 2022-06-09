#![feature(explicit_generic_args_with_impl_trait)]

extern crate core;

use std::any::{Any};
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::component::{Components, Fetch};
use crate::entity_builder::{EntityBuilder, EntityId};
use crate::resource::Resources;
use crate::system::{System, Systems, Type};

mod entity_builder;
mod system;
mod component;
mod resource;


#[derive(Default)]
pub struct World<C> {
    resources: Resources,
    components: Components,
    systems: Systems<C>,
}

pub fn builder<C>() -> WorldBuilder<C> {
    WorldBuilder {
        components: Default::default(),
        systems: Systems {
            items: HashMap::new()
        },
    }
}

#[derive(Default)]
pub struct WorldBuilder<C> {
    components: Components,
    systems: Systems<C>,
}

impl<C> WorldBuilder<C> {
    pub fn with_system<T>(mut self, system_type: Type, f: impl Fn(&mut C, <T as Fetch<'_>>::Data) + 'static) -> Self
        where
                for<'a> T: Fetch<'a> + 'static,
                // for<'a> F: Fn(&mut C, <T as Fetch<'_>>::Data) + 'static
    {
        for (id, name) in T::type_info() {
            if !self.components.items.contains_key(&id) {
                println!("Registering {name}");
                self.components.items.insert(id, vec![]);
            }
        }

        let system = System {
            f,
            t: PhantomData,
        };

        self.systems.items.entry(system_type)
            .or_insert_with(Vec::new)
            .push(Box::new(system));

        self
    }


    pub fn build(self) -> World<C> {
        World {
            resources: Resources::default(),
            components: self.components,
            systems: self.systems,
        }
    }
}

impl<C> World<C> {
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

    pub fn get_component<T: Any>(&mut self, entity_id: EntityId) -> Option<&mut T> {
        self.components.get_component(entity_id)
    }

    pub fn run_systems(&mut self, system_type: Type, ctx: &mut C) {
        let components = &mut self.components;
        if let Some(systems) = self.systems.items.get_mut(&system_type) {
            for system in systems.iter_mut() {
                system.run(ctx, components)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::marker::PhantomData;
    use crate::component::Component;

    use super::*;

    #[derive(Debug, Default, Eq, PartialEq)]
    struct Speed(u32);

    #[derive(Debug, Default, Eq, PartialEq)]
    struct Health(u32);


    #[test]
    fn test_resource() {
        struct WorldWidth(u32);

        let mut world = builder::<()>().build();
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


    fn run(ctx: &mut Ctx, (speed, health): (&mut Speed, &mut Health)) {
        speed.0 += 1;
        println!("{ctx:?} {speed:?} {health:?}");
    }

    #[derive(Debug)]
    struct Ctx<'a> {
        t: PhantomData<&'a ()>,
    }

    #[test]
    fn test_run_system() {
        let mut world = builder()
            .with_system::<(Speed, Health)>(Type::Default, run)
            .build();

        world.new_entity()
            .with_component(Health(100))
            .with_component(Speed(1));

        let mut ctx = Ctx { t: PhantomData };
        world.run_systems(Type::Default, &mut ctx);
    }
}