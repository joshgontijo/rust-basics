use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::process::id;
use std::ptr;
use std::rc::Rc;

type EntityId = usize;

#[derive(Default)]
pub struct Components {
    entities: usize,
    items: HashMap<TypeId, Vec<Option<Box<dyn Any>>>>,
    vacant: VecDeque<usize>,
}

pub struct Component<T: Any> {
    inner: Box<dyn Any>,
    _m: PhantomData<T>,
}


impl Components {
    pub(crate) fn new(items: HashMap<TypeId, Vec<Option<Box<dyn Any>>>>) -> Self {
        Self { entities: 0, items, vacant: VecDeque::default() }
    }

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

    pub fn get_component_mut<T: Any>(&mut self, entity_id: EntityId) -> Option<&mut T> {
        let component = self.items.get_mut(&TypeId::of::<T>())
            .unwrap()
            .get_mut(entity_id)?;
        match component {
            None => None,
            Some(c) => Some(c.downcast_mut().unwrap())
        }
    }

    pub fn query<'a, Q: Query<'a>>(&'a mut self, entity_id: EntityId) -> Option<Q::Data> {
        Q::get_component(self, entity_id)
    }
}


pub trait Query<'a> {
    type Data;
    fn get_component(components: &'a mut Components, entity_id: EntityId) -> Option<Self::Data>;
}

impl<'a, T1, T2> Query<'a> for (T1, T2)
    where
        T1: Any,
        T2: Any,
{
    type Data = (&'a mut T1, &'a mut T2);

    fn get_component(components: &'a mut Components, entity_id: EntityId) -> Option<Self::Data> {
        unsafe {
            let t1 = components.get_component_mut::<T1>(entity_id)? as *mut _;
            let t2 = components.get_component_mut::<T2>(entity_id)? as *mut _;
            Some((&mut *t1, &mut *t2))
        }
        // unsafe {
        //     get_mut_ref(components, entity_id)?
        //     let a = components.items.get_mut(a).unwrap().get_mut(idx) as *mut _;
        //     let b = components.items.get_mut(b).unwrap() as *mut _;
        //     assert_ne!(a, b, "The two keys must not resolve to the same value");
        //     (&mut *a, &mut *b)
        // }


        // unsafe {
        //     let a = components.items.get_mut(a).unwrap().get_mut(idx) as *mut _;
        //     let b = components.items.get_mut(b).unwrap() as *mut _;
        //     assert_ne!(a, b, "The two keys must not resolve to the same value");
        //     (&mut *a, &mut *b)
        // }
    }
}
