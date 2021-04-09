use std::borrow::Borrow;
use std::collections::{HashMap, BTreeMap};
use std::sync::RwLock;

fn main() {
    let user = User { age: 10 };
    let multiply = |user: &User| -> u32 { user.age * 2 };
    dbg!(apply_fn(&user, &multiply));
    dbg!(apply_fn(&user, &multiply));

    let closure = returns_closure();
    let res = dbg!(apply_fn(&user, closure.borrow()));

    let mut map = HashMap::<i32, String>::new();
    map.entry(1).or_insert_with(|| "".to_string());

}

fn apply_fn(user: &User, fun: &dyn Fn(&User) -> u32) -> u32 {
    fun(user)
}

fn returns_closure() -> Box<dyn Fn(&User) -> u32> {
    Box::new(|x| x.age + 1)
}



struct User {
    age: u32
}