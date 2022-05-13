use std::any::{Any, TypeId};
use std::collections::{HashMap};

pub type EntityId = usize;

pub struct EntityBuilder<'a> {
    pub id: EntityId,
    pub(crate) components: &'a mut HashMap<TypeId, Vec<Option<Box<dyn Any>>>>,
}

impl<'a> EntityBuilder<'a> {
    pub fn with_component<T: Any>(&'a mut self, component: T) -> &'a mut Self {
        let component_vec = self.components.get_mut(&TypeId::of::<T>()).expect("Component typs not registered");
        component_vec.insert(self.id, Some(Box::new(component)));
        self
    }

    pub fn id(&mut self) -> EntityId {
        self.id
    }
}