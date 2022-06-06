use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::component::{Component, Components};

mod component;


fn main() {
    let mut items = HashMap::<TypeId, Vec<Option<Box<dyn Any>>>>::new();
    items.insert(TypeId::of::<u32>(), vec![Some(Box::new(123))]);
    items.insert(TypeId::of::<String>(), vec![Some(Box::new("ABC".to_string()))]);

    let mut components = Components::new(items);

    let found = components.query::<(u32, String)>(0);
    println!("{found:?}");

    let (int, string) = found.unwrap();

    *int = 2;


}