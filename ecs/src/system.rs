use std::marker::PhantomData;
use std::slice::{Iter, IterMut};
use crate::{Components, Fetch};

struct System<T> where T: Fetch {
    f: fn(T::Data),
    _m: PhantomData<T>,
}

impl<T> System<T> where T: Fetch {
    fn new(f: fn(T::Data)) -> Self {
        Self {
            f,
            _m: PhantomData,
        }
    }



}

#[derive(Default)]
pub struct Systems {
    items: Vec<Box<dyn SystemRunner + 'static>>,
}

impl Systems {
    pub fn add_system<T>(&mut self, f: fn(T::Data))
        where
            T: Fetch + 'static
    {
        self.items.push(Box::new(System::<T>::new(f)));
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Box<dyn SystemRunner>> {
        self.items.iter_mut()
    }

}

pub trait SystemRunner {
    fn run(&mut self, components: &Components);
}



impl<T: Fetch> SystemRunner for System<T> {
    fn run(&mut self, components: &Components) {
        let mut iter = components.query::<T>();
        while let Some(item) = iter.next() {
            (self.f)(item);
        }
    }
}

