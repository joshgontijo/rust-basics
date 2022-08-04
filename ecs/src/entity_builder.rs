use std::any::{Any};
use crate::component::Components;


pub type EntityId = usize;

pub struct EntityBuilder<'a> {
    pub(crate) id: EntityId,
    pub(crate) components: &'a mut Components,
}

impl<'a> EntityBuilder<'a> {
    pub fn with_component<T: Any>(&'a mut self, component: T) -> &'a mut Self {
        self.components.add_component(self.id, component);
        self
    }

    pub fn id(&mut self) -> EntityId {
        self.id
    }
}