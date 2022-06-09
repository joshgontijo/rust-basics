use std::marker::PhantomData;
use std::slice::{Iter, IterMut};
use crate::{Components, Fetch};

pub(crate) struct System<C, F, T> {
    pub(crate) f: F,
    pub(crate) _m1: PhantomData<fn() -> T>,
    pub(crate) _m2: PhantomData<fn() -> C>
}

#[derive(Default)]
pub struct Systems<C> {
    pub(crate)items: Vec<Box<dyn SystemRunner<C>>>,
}

impl<C> Systems<C> {

    pub fn run(&mut self, ctx: &mut C, components: &mut Components) {
        for system in self.items.iter_mut() {
            system.run(ctx, components)
        }
    }
}

pub trait SystemRunner<C> {
    fn run(&mut self, ctx: &mut C, components: &mut Components);
}


impl<C, T, F> SystemRunner<C> for System<C, F, T>
    where
            for<'a> T: Fetch<'a>,
            for<'a> F: Fn(&mut C, <T as Fetch<'a>>::Data)

{
    fn run(&mut self, ctx: &mut C, components: &mut Components) {
        for entity_id in 0..components.entities {
            if let Some(comp) = T::fetch(components, entity_id) {
                (self.f)(ctx, comp);
            }
        }
    }
}

