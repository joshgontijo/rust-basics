//https://www.reddit.com/r/learnrust/comments/103unuk/implementing_a_heterogeneous_collection_of/?utm_source=share&utm_medium=android_app&utm_name=androidcss&utm_term=10&utm_content=share_button

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::marker::PhantomData;

fn main() {
    let mut container = Container::default();
    //now we can add multiple generic implementation types !!
    container.add(ExampleA);
    container.add(ExampleB);

    //we must be careful to not use the wrong one here
    //type annotation must match the ExampleA signature
    let res: String = container.call(12345_u32);
    println!("{res}");

    //type annotation must match the ExampleB signature
    let res: String = container.call(16666_i64);
    println!("{res}");
}

//------------- EXAMPLE usage, not important
struct ExampleA;

impl SomeGenericTrait<u32, String> for ExampleA {
    fn do_something(&self, t: u32) -> String {
        return format!("ExampleA: {t}");
    }
}

struct ExampleB;

impl SomeGenericTrait<i64, String> for ExampleB {
    fn do_something(&self, t: i64) -> String {
        return format!("ExampleB {t}");
    }
}


//---------------- INTERNAL / HELPER traits just to deal with the type system

//internal trait, this trait must not be generic as it will be our 'common denominator' type
//It somewhat replicate type type signature of the trait we want to store, but we replace any generic type with Box<Any>
//to allow us to downcast it internally
trait ErasedTrait {
    fn do_something_dyn(&self, t: Box<dyn Any>) -> Box<dyn Any>;
}

//wrapper struct that will hold the generic struct E and all its generic parameters
struct ErasedTypeImpl<E, T, R>
    where
        E: SomeGenericTrait<T, R>,
        T: Any,
        R: Any
{
    //just a phantom data, the type signature is just following best practices,
    // it does not represent the actual fn signature of the 'do_something'
    _ph: PhantomData<fn(T) -> R>,
    item: E,
}

//implement the internal trait to our erased internal struct
//Here we will handle the type conversion
impl<E, T, R> ErasedTrait for ErasedTypeImpl<E, T, R>
    where
        E: SomeGenericTrait<T, R>,
        T: Any,
        R: Any
{
    fn do_something_dyn(&self, t: Box<dyn Any>) -> Box<dyn Any> {
        let downcast = *t.downcast::<T>().unwrap();//downcast
        let result = self.item.do_something(downcast); //process
        return Box::new(result); //box/erase
    }
}


//--------------------- PUBLIC STUFF -----------

//pub trait
pub trait SomeGenericTrait<T, R> {
    fn do_something(&self, t: T) -> R;
}

//this container holds many SomeGenericTrait<T,R> where T and R can be of different types
//in Java (and other languages) this is similar of an List<GenericClass<Object>>
#[derive(Default)]
pub struct Container {
    items: HashMap<(TypeId, TypeId), Box<dyn ErasedTrait>>,
}

impl Container {
    pub fn add<E, T, R>(&mut self, item: E)
        where
            E: SomeGenericTrait<T, R> + 'static,
            T: Any,
            R: Any
    {
        let key = (TypeId::of::<T>(), TypeId::of::<R>());
        let wrapped = Box::new(ErasedTypeImpl {
            _ph: PhantomData,
            item,
        });

        self.items.insert(key, wrapped);
    }


    pub fn call<T, R>(&self, input: T) -> R
        where
            T: Any,
            R: Any
    {
        let key = (TypeId::of::<T>(), TypeId::of::<R>());
        if !self.items.contains_key(&key) {
            panic!("Entry not found");
        }


        let item = self.items.get(&key).unwrap();
        let result = item.do_something_dyn(Box::new(input));
        *result.downcast::<R>().unwrap()
    }
}



