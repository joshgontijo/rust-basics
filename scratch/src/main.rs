use crate::component::Components;
use crate::system::Systems;

mod system;
mod component;

pub type EntityId = usize;

struct Ctx<'a> {
    value: &'a str
}

fn main() {

    let val = "a".to_string();

    let mut ctx = Ctx {
        value: val.as_str()
    };

    let mut components = Components::default();
    let mut systems = Systems::<Ctx>::new();

    for system in systems.iter_mut() {
        system.run(&mut ctx, &mut components);
    }

}