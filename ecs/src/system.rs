use std::collections::HashMap;
use std::marker::PhantomData;
use crate::{Components, Fetch};

pub(crate) struct System<F, T> {
    pub(crate) f: F,
    pub(crate) t: PhantomData<fn(T)>,

}


#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum Type {
    Default,
    Render,
    Custom(&'static str),
}


// impl<C, T> System<C, T>
//     where for<'a>
//           T: Fetch<'a>,
// {
//     pub(crate) fn new(f: fn(&mut C, <T as Fetch<'_>>::Data)) -> Self {
//         Self {
//             f,
//             _m1: PhantomData::<T>::default(),
//             _m2: PhantomData::<C>::default(),
//         }
//     }
// }

#[derive(Default)]
pub struct Systems<C> {
    pub(crate) items: HashMap<Type, Vec<Box<dyn SystemRunner<C>>>>,
}

pub trait SystemRunner<C> {
    fn run(&mut self, ctx: &mut C, components: &mut Components);
}


impl<C, F, T> SystemRunner<C> for System<F, T>
    where
            for<'a> T: Fetch<'a>,
            for<'a> F: Fn(&mut C, <T as Fetch<'_>>::Data)
{
    fn run(&mut self, ctx: &mut C, components: &mut Components) {
        for entity_id in 0..components.entities {
            if let Some(comp) = T::fetch(components, entity_id) {
                (self.f)(ctx, comp);
            }
        }
    }
}

