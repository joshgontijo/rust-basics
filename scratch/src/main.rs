use std::rc::Rc;

fn main() {
    let data = Rc::new(&[&[0, 1, 2], &[3, 4, 5]]);

    data.iter()
        .flat_map(|i| i.iter())
        .for_each(|e| println!("{}", e));
}


