use std::marker::PhantomData;
use std::slice::{Iter, IterMut};
use crate::{Components, Fetch};

struct System<C, T>
    where for<'a>
          T: Fetch<'a>,
{
    f: fn(&mut C, <T as Fetch<'_>>::Data),
    _m1: PhantomData<T>,
    _m2: PhantomData<C>,
}

impl<C, T> System<C, T>
    where for<'a>
        T: Fetch<'a>,
{
    fn new(f: fn(&mut C, <T as Fetch<'_>>::Data)) -> Self {
        Self {
            f,
            _m1: PhantomData::<T>::default(),
            _m2: PhantomData::<C>::default(),
        }
    }
}

#[derive(Default)]
pub struct Systems<C> {
    items: Vec<Box<dyn SystemRunner<C>>>,
}

impl<C: 'static> Systems<C> {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn add_system<T>(&mut self, f: fn(&mut C, <T as Fetch<'_>>::Data))
        where for<'a>
            T: Fetch<'a> + 'static,
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


impl<C, T> SystemRunner<C> for System<C, T>
    where for<'a>
          T: Fetch<'a>,
{
    fn run(&mut self, ctx: &mut C, components: &Components) {
        let mut iter = components.query::<T>();
        while let Some(item) = iter.next() {
            (self.f)(ctx, item);
        }
    }
}

