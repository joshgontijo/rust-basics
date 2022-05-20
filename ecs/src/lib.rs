#![feature(type_alias_impl_trait)]

extern crate core;

use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

use crate::entity_builder::{EntityBuilder, EntityId};

mod entity_builder;

#[derive(Debug, Default, Eq, PartialEq)]
struct Speed(u32);

#[derive(Debug, Default, Eq, PartialEq)]
struct Health(u32);

#[derive(Debug, Default)]
pub struct World {
    entity_count: usize,
    components: HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any>>>>>,
    resources: HashMap<TypeId, Box<dyn Any>>,
}


#[derive(Default)]
pub struct WorldBuilder {
    components: HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any>>>>>,
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

    fn get_component<T: Any>(&self, entity_id: EntityId) -> Option<Component<T>> {
        self.components.get(&TypeId::of::<T>())
            .expect("Component not registered")
            .get(entity_id)
            .map(|e| {
                match e {
                    None => None,
                    Some(t) => Some(Component::new(Rc::clone(t)))
                }
            }).flatten()
    }

    fn query<Tuple>(&self) -> ComponentsIter<Tuple> {
        ComponentsIter {
            entity_idx: 0,
            world: self,
            _m: PhantomData,
        }
    }
}

struct Component<T: Any> {
    inner: Rc<RefCell<dyn Any>>,
    _m: PhantomData<T>,
}

impl<T: Any + Debug> Debug for Component<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<T: Any> Component<T> {
    fn new(rc: Rc<RefCell<dyn Any>>) -> Self {
        Self {
            inner: rc,
            _m: PhantomData,
        }
    }

    // fn as_ref(& self) -> impl Deref<Target = T> + '_ {
    //     Ref::map(self.inner.borrow(), |any| any.downcast_ref::<T>().unwrap())
    // }

    fn as_ref(&self) -> Ref<'_, T> {
        Ref::map(self.inner.borrow(), |any| any.downcast_ref::<T>().unwrap())
    }

    fn as_ref_mut(&self) -> RefMut<'_, T> {
        RefMut::map(self.inner.borrow_mut(), |any| any.downcast_mut::<T>().unwrap())
    }
}

struct ComponentsIter<'a, Tuple> {
    entity_idx: usize,
    world: &'a World,
    _m: PhantomData<Tuple>,
}

impl<'a, Tuple: Fetch> Iterator for ComponentsIter<'a, Tuple> {
    type Item = Tuple::Data;

    fn next(&mut self) -> Option<Self::Item> {
        let world = &mut self.world;
        let res = Tuple::fetch(world, self.entity_idx);
        self.entity_idx += 1;
        res
    }
}

trait Fetch {
    type Data;
    fn fetch(world: &World, idx: usize) -> Option<Self::Data>;
}

/// Resolves to:
/// impl<T1, T2> Fetch for (T1, T2)
///    where
///       T1: Any,
///       T2: Any
///  {
///      type Data = (Component<T1>, Component<T2>);
///
///      fn fetch(world: &World, idx: usize) -> Option<Self::Data> {
///          Some((world.get_component::<T1>(idx)?, world.get_component::<T2>(idx)?))
///      }
///  }
macro_rules! fetch_tuple {

     ($($ty: ident),*) => {// match like arm for macro
          impl<$($ty,)*> Fetch for ($($ty,)*)
            where
                $(
                    $ty: Any,
                )*

         {
            type Data = ($(Component<$ty>,)*);

            fn fetch(world: &World, idx: usize) -> Option<Self::Data> {
                // let t1 = world.get::<T1>(idx);
                // let t2 = world.get::<T2>(idx);
                // let res = ( world.get::<T1>(idx)?, world.get::<T2>(idx)?);
                // return Some(res);

                Some(($(world.get_component::<$ty>(idx)?,)*))
                }
         }
    }
}


fetch_tuple! {T0}
fetch_tuple! {T0, T1}
fetch_tuple! {T0, T1, T2}
fetch_tuple! {T0, T1, T2, T3}
fetch_tuple! {T0, T1, T2, T3, T4}
fetch_tuple! {T0, T1, T2, T3, T4, T5}
fetch_tuple! {T0, T1, T2, T3, T4, T5, T6}
fetch_tuple! {T0, T1, T2, T3, T4, T5, T6, T7}
fetch_tuple! {T0, T1, T2, T3, T4, T5, T6, T7, T8}
fetch_tuple! {T0, T1, T2, T3, T4, T5, T6, T7, T8, T9}


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


        let component = world.get_component::<Speed>(0);
        assert!(component.is_some());
        assert_eq!(component.unwrap().as_ref().0, 1);
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