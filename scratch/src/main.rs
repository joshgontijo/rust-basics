use serde::{Deserialize, Serialize};

fn main() {
    let user = User { age: 31, value: 123 };
    let mut result = bincode::serialize(&user).unwrap();

    result.append(&mut vec![1u8; 1024]);

    let read_buff = &result[..];

    let read_user: User<i32> = bincode::deserialize(read_buff).unwrap();

    println!("{:?}", read_user);
}

#[derive(Debug, Serialize, Deserialize)]
struct User<T> {
    age: u32,
    value: T,
}

