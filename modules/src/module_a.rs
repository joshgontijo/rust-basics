use crate::module_b::say_world;

pub fn hello_world() -> String {
    "Hello ".to_owned() + say_world()
}