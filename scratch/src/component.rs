use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::process::id;
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
        let vec = self.items.get_mut(&TypeId::of::<T>())?;
        let component = vec.get_mut(entity_id)?;
        match component {
            None => None,
            Some(c) => {
                Some(c.downcast_mut::<T>()).unwrap()
            }
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
        T1: Any + PartialEq + Debug,
        T2: Any + PartialEq + Debug,
{
    type Data = (&'a mut T1, &'a mut T2);

    fn get_component(components: &'a mut Components, entity_id: EntityId) -> Option<Self::Data> {
        //TODO add entity_id bound check
        unsafe {
            // let value = components.items.get_mut(&TypeId::of::<T1>()).unwrap().get_mut(entity_id).unwrap();
            // let g: Option<&mut T1> = match value {
            //     None => None,
            //     Some(b) => Some(b.downcast_mut::<T1>().unwrap())
            // };
            // let x1 = g.unwrap();


            let mut a = components.items.get_mut(&TypeId::of::<T1>()).unwrap() as *mut Vec<Option<Box<dyn Any>>>;
            let mut b = components.items.get_mut(&TypeId::of::<T2>()).unwrap() as *mut Vec<Option<Box<dyn Any>>>;
            assert_ne!(a, b, "The two keys must not resolve to the same value");

            let avec = (*a).get_mut(entity_id).unwrap();
            let res1 = match avec {
                None => None,
                Some(value) =>  {
                    let x = value.downcast_mut();
                    Some(x.unwrap())
                }
            };

            let bvec = (*b).get_mut(entity_id).unwrap();
            let res2 = match bvec {
                None => None,
                Some(value) => Some(value.as_mut().downcast_mut::<T2>().unwrap())
            };

            return Some((res1?, res2?))
        }


        todo!()
        // unsafe {
        //     let a = components.items.get_mut(a).unwrap().get_mut(idx) as *mut _;
        //     let b = components.items.get_mut(b).unwrap() as *mut _;
        //     assert_ne!(a, b, "The two keys must not resolve to the same value");
        //     (&mut *a, &mut *b)
        // }
    }
}
