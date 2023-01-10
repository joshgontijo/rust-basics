use std::any::{Any, TypeId};
use std::marker::PhantomData;

use crate::component::{Components, Query, Fetch};
use crate::entity_builder::{EntityBuilder, EntityId};
use crate::resource::Resources;

#[derive(Default)]
pub struct World {
    pub resources: Resources,
    pub components: Components,
}

pub fn builder() -> WorldBuilder {
    WorldBuilder {
        components: Default::default(),
    }
}

#[derive(Default)]
pub struct WorldBuilder {
    components: Components,
}

impl WorldBuilder {
    pub fn register<C: ?Sized + 'static>(mut self) -> Self {
        let type_id = TypeId::of::<C>();
        if !self.components.items.contains_key(&type_id) {
            let name = std::any::type_name::<C>();
            println!("Registering {name}");
            self.components.items.insert(type_id, vec![]);
        }

        self
    }


    pub fn build(self) -> World {
        World {
            resources: Resources::default(),
            components: self.components,
        }
    }
}

impl World {
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
        EntityBuilder {
            id: entity_id,
            components: &mut self.components,
        }
    }

    pub fn get_component<T: Any>(&mut self, entity_id: EntityId) -> Option<&mut T> {
        self.components.get_component(entity_id)
    }


    pub fn query<Tuple>(&mut self) -> Query<Tuple> {
        self.components.query::<Tuple>()
    }

    pub fn run_system<C, T>(&mut self, f: fn(<T as Fetch<'_>>::Data))
        where
            T: for<'a> Fetch<'a>,
    {
        for entity_id in 0..self.components.entities {
            if let Some(component) = T::fetch(&mut self.components, entity_id) {
                (f)(component)
            }
        }
    }
    
    pub fn run_system_with_context<C, T>(&mut self, ctx: &mut C, f: fn(&mut C, <T as Fetch<'_>>::Data))
        where
            T: for<'a> Fetch<'a>,
    {
        for entity_id in 0..self.components.entities {
            if let Some(component) = T::fetch(&mut self.components, entity_id) {
                (f)(ctx, component)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::marker::PhantomData;
    use crate::component::LendingIterator;

    use super::*;

    #[derive(Debug, Default, Eq, PartialEq)]
    struct Speed(u32);

    #[derive(Debug, Default, Eq, PartialEq)]
    struct Health(u32);


    #[test]
    fn test_resource() {
        struct WorldWidth(u32);

        let mut world = builder().build();
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


    fn example_system(ctx: &mut Ctx, (speed, health): (&mut Speed, &mut Health)) {
        speed.0 += 1;
        println!("{ctx:?} {speed:?} {health:?}");
    }

    #[derive(Debug)]
    struct Ctx<'a> {
        t: PhantomData<&'a ()>,
    }

    #[test]
    fn test_run_system() {
        let mut ctx = Ctx { t: PhantomData };

        let mut world = builder()
            .register::<Speed>()
            .register::<Health>()
            .build();

        world.new_entity()
            .with_component(Health(100))
            .with_component(Speed(1));


       world.run_system_with_context::<_, (Speed, Health)>(&mut ctx, example_system);

    }

    #[test]
    fn test_run_query() {
        let mut ctx = Ctx { t: PhantomData };

        let mut world = builder()
            .register::<Speed>()
            .register::<Health>()
            .build();

        world.new_entity()
            .with_component(Health(100))
            .with_component(Speed(1));


        my_system(&mut ctx, world.query());

    }

    fn my_system(ctx: &mut Ctx, mut iter: Query<(Speed, Health)>) {
        while let Some((speed, health)) = iter.next() {
            println!("{speed:?} {health:?}");
        }
    }

}