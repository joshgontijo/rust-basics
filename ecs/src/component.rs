use std::any::{Any, TypeId};
use std::collections::{HashMap, VecDeque};
use std::marker::PhantomData;

use crate::entity_builder::EntityId;

#[derive(Default)]
pub struct Components {
    pub(crate) entities: usize,
    //TODO use vec<usize> instead, this can cause issues after removing entities
    pub(crate) items: HashMap<TypeId, Vec<Option<Box<dyn Any>>>>,
    vacant: VecDeque<usize>,
}

pub struct Component<T: Any> {
    inner: Box<dyn Any>,
    _m: PhantomData<T>,
}


impl Components {
    pub fn new_entity(&mut self) -> EntityId {
        match self.vacant.pop_front() {
            None => { //alocate new one
                let idx = self.entities;
                self.items.values_mut().for_each(|components| components.push(None));
                self.entities += 1;
                idx
            }
            Some(vacant) => vacant
        }
    }

    pub fn remove_entity(&mut self, id: EntityId) {
        if id >= self.entities {
            panic!("Entity id out of bounds")
        }
        for (_, components) in self.items.iter_mut() {
            components.insert(id, None)
        }
        self.vacant.push_back(id);
        self.entities -= 1;
    }

    pub fn remove_component<T: Any>(&mut self, id: EntityId) {
        if id >= self.entities {
            panic!("Entity id out of bounds")
        }
        let type_id = TypeId::of::<T>();
        self.items.get_mut(&type_id)
            .expect("Component not registered")
            .insert(id, None);
    }

    pub fn add_component<T: Any>(&mut self, entity_id: EntityId, component: T) {
        let component_vec = self.items.get_mut(&TypeId::of::<T>()).expect("Component type not registered");
        component_vec.insert(entity_id, Some(Box::new(component)));
    }

    pub fn get_component<T: Any>(&mut self, entity_id: EntityId) -> Option<&mut T> {
        let component = self.items.get_mut(&TypeId::of::<T>())
            .unwrap()
            .get_mut(entity_id)?;
        match component {
            None => None,
            Some(c) => Some(c.downcast_mut().unwrap())
        }
    }


    pub fn query<Tuple>(&mut self) -> Query<Tuple> {
        Query {
            entity_idx: 0,
            components: self,
            _m: PhantomData,
        }
    }
}

pub trait LendingIterator {
    type Item<'a> where Self: 'a;
    fn next(&mut self) -> Option<Self::Item<'_>>;

    fn for_each<F>(mut self, mut f: F)
        where
            Self: Sized,
            F: FnMut(Self::Item<'_>)
    {
        while let Some(c) = self.next() {
            f(c)
        }
    }
}

pub struct Query<'a, Tuple> {
    entity_idx: usize,
    components: &'a mut Components,
    _m: PhantomData<Tuple>,
}

impl<'iter, Tuple> LendingIterator for Query<'iter, Tuple>
    where
        Tuple: for<'b> Fetch<'b>
{
    type Item<'a>  = <Tuple as Fetch<'a>>::Data where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        let entities = self.components.entities;
        while self.entity_idx < entities {
            if let Some(comp) = Tuple::fetch(&mut self.components, self.entity_idx) {
                self.entity_idx += 1;
                return Some(comp);
            }
            self.entity_idx += 1;
        }
        return None;
    }
}


pub trait Fetch<'a> {
    type Data;
    fn fetch(components: &mut Components, entity_id: EntityId) -> Option<Self::Data>;
    fn type_info() -> Vec<(TypeId, &'static str)>;
}

macro_rules! fetch_tuple {

     ($($ty: ident),*) => {// match like arm for macro
          impl<'a, $($ty,)*> Fetch<'a> for ($($ty,)*)
            where
                $(
                    $ty: Any,
                )*

         {
            type Data = ($(&'a mut $ty,)*);

            fn fetch(components: &mut Components, entity_id: usize) -> Option<Self::Data> {
               unsafe {
                    Some((
                         $(&mut *(components.get_component::<$ty>(entity_id)? as *mut _),)*
                    ))
               }
            }
             
             fn type_info() -> Vec<(TypeId, &'static str)> {
                vec![
                    $((TypeId::of::<$ty>(), std::any::type_name::<$ty>()),)*
                ]
            }
             
         }
    }
}


fetch_tuple! {}
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