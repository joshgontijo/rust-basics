use std::any::{Any, TypeId};
use std::collections::HashSet;

pub struct Query {}

#[derive(Default, Debug)]
pub struct QueryBuilder {
    types: HashSet<TypeId>,
}

impl QueryBuilder {
    pub fn with_component<T: Any>(mut self) -> Self {
        self.types.insert(TypeId::of::<T>());
        Self {
            types: self.types
        }
    }

    pub fn run(self) -> Query {
        Query {}
    }
}
