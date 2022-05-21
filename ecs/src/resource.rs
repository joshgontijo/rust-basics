use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Default)]
pub struct Resources {
    items: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    pub fn add_resource<T: Any>(&mut self, resource: T) -> &mut Self {
        self.items.insert(TypeId::of::<T>(), Box::new(resource));
        self
    }

    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.items.get(&TypeId::of::<T>()).map(|v| {
            v.downcast_ref::<T>().unwrap()
        })
    }

    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.items.get_mut(&TypeId::of::<T>()).map(|v| {
            v.downcast_mut::<T>().unwrap()
        })
    }

    pub fn remove_resource<T: Any>(&mut self) -> Option<T> {
        self.items.remove(&TypeId::of::<T>())
            .map(|v| {
                *v.downcast::<T>().unwrap()
            })
    }
}