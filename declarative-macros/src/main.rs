macro_rules! print_tuple_types {

     ($($ty: ident),*) => {// match like arm for macro
       impl<$($ty),*> Print for ($($ty,)*)
        {
           fn print() {
               $( // repeat for each enclosing identifier (repeat for each $ty)
                    println!("TYPE: {}", std::any::type_name::<$ty>());
                )*
           }
        }
    }
}

trait Print {
    fn print();
}

print_tuple_types!{T1}
print_tuple_types!{T1, T2}
print_tuple_types!{T1, T2, T3}

fn main() {

    <(Speed,Health)>::print();

}

#[derive(Debug)]
struct Speed(u32);

#[derive(Debug)]
struct Health(u32);
