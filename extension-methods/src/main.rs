use std::fmt::Debug;

fn main() {
    "Hello, world!".debug();
    1.debug();
}


trait StringExt: Debug {
    fn debug(&self) {
        println!("[DEBUG] {self:?}");
    }
}

impl<A: Debug> StringExt for A{}