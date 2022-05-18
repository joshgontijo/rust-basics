use std::alloc::Layout;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::slice::{Iter, IterMut};

// impl<T> Print for (T) {
//     fn print() {
//         println!("TYPE: {}", std::any::type_name::<T>())
//     }
// }

// macro_rules! print_tuple_types {
//
//      ($($ty: ident),*) => {// match like arm for macro
//        impl<$($ty),*> Print for ($($ty,)*)
//         {
//            fn print() {
//                $( // repeat for each enclosing identifier (repeat for each $ty)
//                     println!("TYPE: {}", std::any::type_name::<$ty>());
//                 )*
//            }
//         }
//     }
// }

macro_rules! fetch_tuple {

     ($($ty: ident),*) => {// match like arm for macro
          impl<'a, $($ty,)*> Fetch<'a> for ($($ty,)*)
            where
                $(
                    $ty: Any,
                )*

         {
            type Data = ($(&'a $ty,)*);

            fn fetch(world: &'a World, idx: usize) -> Option<Self::Data> {
                // let t1 = world.get::<T1>(idx);
                // let t2 = world.get::<T2>(idx);
                // let res = ( world.get::<T1>(idx)?, world.get::<T2>(idx)?);
                // return Some(res);
                Some(($(world.get::<$ty>(idx)?,)*))
                }
         }
    }
}


trait Print {
    fn print();
}

// print_tuple_types! {T1}
// print_tuple_types! {T1, T2}
// print_tuple_types! {T1, T2, T3}


fetch_tuple! {T1}
fetch_tuple! {T1, T2}

fn main() {
    let mut world = World::default();
    {
        let mut map = &mut world.map;
        let speed = TypeId::of::<Speed>();
        let health = TypeId::of::<Health>();
        map.insert(speed, vec![]);
        map.insert(health, vec![]);

        map.get_mut(&speed).unwrap().push(Some(Box::new(Speed(1))));
        // map.get_mut(&speed).unwrap().push(Some(Box::new(Speed(5))));
        map.get_mut(&speed).unwrap().push(None);

        map.get_mut(&health).unwrap().push(Some(Box::new(Health(2))));
        map.get_mut(&health).unwrap().push(Some(Box::new(Health(3))));
    }


    let mut it = world.iter::<(Speed, Health)>();
    while let Some((speed, health)) = it.next() {
        println!("{speed:?} - {health:?}");
    }

    println!("--------");

    let mut it = world.iter::<(Health,)>();
    while let Some((health)) = it.next() {
        println!("{health:?}");
    }
}


trait Fetch<'a> {
    type Data;
    fn fetch(world: &'a World, idx: usize) -> Option<Self::Data>;
}

// impl<'a, T1: Any, T2: Any> Fetch<'a> for (T1, T2) {
//     type Data = (&'a T1, &'a T2);
//
//     fn fetch(world: &'a World, idx: usize) -> Option<Self::Data> {
//         // let t1 = world.get::<T1>(idx);
//         // let t2 = world.get::<T2>(idx);
//         // let res = ( world.get::<T1>(idx)?, world.get::<T2>(idx)?);
//         // return Some(res);
//
//         Some((world.get::<T1>(idx)?, world.get::<T2>(idx)?))
//     }
// }

#[derive(Default, Debug)]
struct World {
    map: HashMap<TypeId, Vec<Option<Box<dyn Any>>>>,
}

struct WorldIter<'a, Tuple> {
    entity_idx: usize,
    world: &'a World,
    _m: PhantomData<Tuple>,
}

impl<'a, Tuple: Fetch<'a>> Iterator for WorldIter<'a, Tuple> {
    type Item = Tuple::Data;

    fn next(&mut self) -> Option<Self::Item> {
        let res = Tuple::fetch(&self.world, self.entity_idx);
        self.entity_idx += 1;
        res
    }
}

impl World {

    fn iter<Tuple>(&self) -> WorldIter<Tuple> {
        WorldIter {
            entity_idx: 0,
            world: &self,
            _m: PhantomData,
        }
    }

    fn get<T: Any>(&self, idx: usize) -> Option<&T> {
        self.map.get(&TypeId::of::<T>())
            .unwrap()
            .get(idx)
            .map(|e| {
                match e {
                    None => None,
                    Some(t) => Some(t.downcast_ref::<T>().unwrap())
                }
            }).flatten()
    }
}

#[derive(Debug)]
struct Speed(u32);

#[derive(Debug)]
struct Health(u32);
