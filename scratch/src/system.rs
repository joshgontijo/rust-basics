use std::marker::PhantomData;
use std::slice::{Iter, IterMut};
use crate::{Components};
use crate::component::Fetch;

#[derive(Default)]
pub struct Systems<C> {
    pub(crate) items: Vec<Box<dyn SystemRunner<C>>>,
}

impl<C> Systems<C> {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn add_system<T, F>(&mut self, f: F)
        where for<'a>
              T: Fetch<'a>,
              F: Fn(&mut C, <T as Fetch<'_>>::Data)
    {
        let system = System::<C, T, F>::new(f);
        self.items.push(Box::new(system));
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Box<dyn SystemRunner<C>>> {
        self.items.iter_mut()
    }
}


struct System<C, T, F>
    where
            for<'a> T: Fetch<'a>,
            F: Fn(&mut C, <T as Fetch<'_>>::Data)
{
    f: F,
    _m1: PhantomData<T>,
    _m2: PhantomData<C>,
}

impl<C, T, F> System<C, T, F>
    where
            for<'a> T: Fetch<'a>,
            F: Fn(&mut C, <T as Fetch<'_>>::Data)
{
    fn new(f: F) -> Self {
        Self {
            f,
            _m1: PhantomData::<T>::default(),
            _m2: PhantomData::<C>::default(),
        }
    }
}

pub trait SystemRunner<C> {
    fn run(&mut self, ctx: &mut C, components: &mut Components);
}


impl<C, T, F> SystemRunner<C> for System<C, T, F>
    where for<'a>
          T: Fetch<'a>,
          F: Fn(&mut C, <T as Fetch<'_>>::Data)
{
    fn run(&mut self, ctx: &mut C, components: &mut Components) {
        for entity_id in 0..components.entities {
            if let Some(comp) = T::fetch(components, entity_id) {
                (self.f)(ctx, comp);
            }
        }
    }
}

