use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::process::id;
use std::rc::Rc;

use crate::{EntityId, World};

#[derive(Default)]
pub struct Components {
    entities: usize,
    items: HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any>>>>>,
    vacant: VecDeque<usize>,
}

pub struct Component<T: Any> {
    inner: Rc<RefCell<dyn Any>>,
    _m: PhantomData<T>,
}


impl Components {
    pub(crate) fn new(items: HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any>>>>>) -> Self {
        Self { entities: 0, items, vacant: VecDeque::default() }
    }

    pub fn new_entity(&mut self) -> EntityId {
        match self.vacant.pop_front() {
            None => { //alocate new one
                let idx = self.items.len();
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
        component_vec.insert(entity_id, Some(Rc::new(RefCell::new(component))));
    }

    pub fn get_component<T: Any>(&self, entity_id: EntityId) -> Option<Component<T>> {
        self.items.get(&TypeId::of::<T>())
            .expect("Component not registered")
            .get(entity_id)
            .map(|e| {
                match e {
                    None => None,
                    Some(t) => Some(Component::new(Rc::clone(t)))
                }
            }).flatten()
    }

    pub(crate) fn query<Tuple>(&self) -> ComponentsIter<Tuple> {
        ComponentsIter {
            entity_idx: 0,
            components: self,
            _m: PhantomData,
        }
    }
}


pub struct ComponentsIter<'a, Tuple> {
    entity_idx: usize,
    components: &'a Components,
    _m: PhantomData<Tuple>,
}

impl<'a, Tuple: Fetch> Iterator for ComponentsIter<'a, Tuple> {
    type Item = Tuple::Data;

    fn next(&mut self) -> Option<Self::Item> {
        let components = &mut self.components;
        let res = Tuple::fetch(components, self.entity_idx);
        self.entity_idx += 1;
        res
    }
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

    fn inner_type(&self) -> TypeId {
        TypeId::of::<T>()
    }
    // fn as_ref(& self) -> impl Deref<Target = T> + '_ {
    //     Ref::map(self.inner.borrow(), |any| any.downcast_ref::<T>().unwrap())
    // }

    pub fn as_ref(&self) -> Ref<'_, T> {
        Ref::map(self.inner.borrow(), |any| any.downcast_ref::<T>().unwrap())
    }

    pub fn as_ref_mut(&self) -> RefMut<'_, T> {
        RefMut::map(self.inner.borrow_mut(), |any| any.downcast_mut::<T>().unwrap())
    }
}


pub trait Fetch {
    type Data;
    fn fetch(components: &Components, idx: usize) -> Option<Self::Data>;
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

            fn fetch(components: &Components, idx: usize) -> Option<Self::Data> {
                // let t1 = world.get::<T1>(idx);
                // let t2 = world.get::<T2>(idx);
                // let res = ( world.get::<T1>(idx)?, world.get::<T2>(idx)?);
                // return Some(res);

                Some(($(components.get_component::<$ty>(idx)?,)*))
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