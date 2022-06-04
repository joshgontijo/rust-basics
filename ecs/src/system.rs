use std::marker::PhantomData;
use std::slice::{Iter, IterMut};
use crate::{Components, Fetch, World};

struct System<C, T> where T: Fetch {
    f: Box<dyn Sys<Query = T, Ctx = C>>,
    _m: PhantomData<fn() -> T>,
}

impl<C, T> System<C, T> where T: Fetch {
    fn new(f: Box<dyn Sys<Query = T, Ctx = C>>) -> Self {
        Self {
            f,
            _m: PhantomData,
        }
    }
}

#[derive(Default)]
pub struct Systems<C> {
    pub(crate) items: Vec<Box<dyn SystemRunner<C>>>,
}

impl<C: 'static> Systems<C> {
    pub fn add_system<T>(&mut self, f: Box<dyn Sys<Query = T, Ctx = C>>)
        where
            T: Fetch + 'static
    {
        let system = System::<C, T>::new(f);
        self.items.push(Box::new(system));
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Box<dyn SystemRunner<C>>> {
        self.items.iter_mut()
    }

}

pub trait SystemRunner<C> {
    fn run(&mut self, ctx: &mut C, components: &Components);
}



impl<C, T: Fetch> SystemRunner<C> for System<C, T> {
    fn run(&mut self, ctx: &mut C, components: &Components) {
        let mut iter = components.query::<T>();
        while let Some(item) = iter.next() {
            self.f.run(ctx, item)
        }
    }
}

pub trait Sys {
    type Query: Fetch;
    type Ctx;

    fn run(&self, ctx: &mut Self::Ctx, components: <Self::Query as Fetch>::Data);
}



