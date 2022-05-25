use std::marker::PhantomData;
use std::slice::{Iter, IterMut};
use crate::{Components, Fetch};

struct System<F, C, T>
    where
        T: Fetch,
        F: Fn(&mut C, T::Data)
{
    f: F,
    _m1: PhantomData<T>,
    _m2: PhantomData<C>,
}

impl<F, C, T> System<F, C, T>
    where
        T: Fetch,
        F: Fn(&mut C, T::Data)
{
    fn new(f: F) -> Self {
        Self {
            f,
            _m1: PhantomData::<T>::default(),
            _m2: PhantomData::<C>::default(),
        }
    }
}

#[derive(Default)]
pub struct Systems<C> {
    items: Vec<Box<dyn SystemRunner<C> + 'static>>,
}

impl<C: 'static> Systems<C> {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn add_system<F: 'static, T>(&mut self, f: F)
        where
            T: Fetch + 'static,
            F: Fn(&mut C, T::Data)
    {
        let system = System::<F, C, T>::new(f);
        self.items.push(Box::new(system));
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Box<dyn SystemRunner<C>>> {
        self.items.iter_mut()
    }
}

pub trait SystemRunner<C> {
    fn run(&mut self, ctx: &mut C, components: &Components);
}


impl<F, C, T> SystemRunner<C> for System<F, C, T>
    where
        T: Fetch,
        F: Fn(&mut C, T::Data)
{
    fn run(&mut self, ctx: &mut C, components: &Components) {
        let mut iter = components.query::<T>();
        while let Some(item) = iter.next() {
            (self.f)(ctx, item);
        }
    }
}

