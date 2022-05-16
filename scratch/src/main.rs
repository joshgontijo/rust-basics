#![feature(type_alias_impl_trait)]

use std::alloc::Layout;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;
use std::slice::{Iter, IterMut};



macro_rules! add {
 // macth like arm for macro
    ($a:expr,$b:expr)=>{
 // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            $a+$b
        }
    }
}

fn main() {

    let value = (1,2);



}


fn register_system<T>(system: impl System<Data=T>) {
    
}




struct MovementSystem;
impl System for MovementSystem {
    type Data = (Speed, Health);

    fn run(&self, data: Self::Data) {
        
    }
}

trait System {
    type Data;
    fn run(&self, data: Self::Data);
}

struct World {
    map: HashMap<TypeId, Vec<u32>>,
}

#[derive(Debug)]
struct Speed(u32);

#[derive(Debug)]
struct Health(u32);
