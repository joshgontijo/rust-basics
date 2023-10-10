mod functions;

use std::ops::Deref;
use crate::functions::{SingleArgFnMut, SingleArgFnOnce};

trait LendingIterator {
    type Item<'a> where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;

    fn map<B, F>(self, f: F) -> Map<Self, F>
        where
            Self: Sized,
            F:  for<'a> FnMut(Self::Item<'a>) -> B,
    {
        Map {
            iter: self,
            f,
        }
    }


    fn map_lending<F>(self, f: F) -> Map<Self, F>
        where
            Self: Sized,
            F: for<'a> SingleArgFnMut<Self::Item<'a>>,
    {
        Map {
            iter: self,
            f,
        }
    }


    fn filter<P>(self, predicate: P) -> Filter<Self, P>
        where
            Self: Sized,
            P: for<'a> FnMut(&Self::Item<'a>) -> bool,
    {
        Filter {
            iter: self,
            predicate,
        }
    }

    fn fold<B, F>(mut self, init: B, mut f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item<'_>) -> B,
    {
        let mut accum = init;
        while let Some(x) = self.next() {
            accum = f(accum, x);
        }
        accum
    }

    fn cloned<T>(self) -> Cloned<Self>
        where
            Self: Sized,
            for<'a> Self::Item<'a>: Deref<Target=T>,
            T: Clone,
    {
        Cloned::new(self)
    }
}

pub struct Map<I, F> {
    iter: I,
    f: F,
}

// impl<I, F, R> LendingIterator for Map<I, F>
//     where
//         I: LendingIterator,
//         F: for<'a> FnMut(I::Item<'a>) -> R,
//
// {
//     type Item<'a> = R
//         where Self: 'a;
//
//     fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
//         self.iter.next().map(&mut self.f)
//     }
// }

impl<I, F> LendingIterator for Map<I, F>
    where
        I: LendingIterator,
        F: for<'a> SingleArgFnMut<I::Item<'a>>,
{
    type Item<'a> = <F as SingleArgFnOnce<I::Item<'a>>>::Output
        where
            Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.iter.next().map(&mut self.f)
    }
}


pub struct Cloned<I> {
    iter: I,
}

impl<I> Cloned<I> {
    pub(crate) fn new(iter: I) -> Cloned<I> {
        Cloned { iter }
    }
}

impl<I> LendingIterator for Cloned<I>
    where
        I: LendingIterator,
        for<'a> I::Item<'a>: Deref,
        for<'a> <I::Item<'a> as Deref>::Target: Clone,
{
    type Item<'a> = <I::Item<'a> as Deref>::Target
        where
            Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.iter.next().map(|item| item.deref().clone())
    }
}

pub struct Filter<I, P> {
    iter: I,
    predicate: P,
}

impl<I, P> LendingIterator for Filter<I, P>
    where
        I: LendingIterator,
        P: for<'a> FnMut(&I::Item<'a>) -> bool,
{
    type Item<'a> where Self: 'a = I::Item<'a>;

    fn next(&mut self) -> Option<Self::Item<'_>> {

        //https://blog.rust-lang.org/2022/10/28/gats-stabilization.html#the-borrow-checker-isnt-perfect-and-it-shows
        //https://docs.rs/polonius-the-crab/0.3.1/polonius_the_crab/#the-arcanemagic
        loop {
            let _self = unsafe { &mut *(self as *mut Self) };
            if let Some(item) = _self.iter.next() {
                if (_self.predicate)(&item) {
                    return Some(item);
                }
            } else {
                return None;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::io::{Cursor, Read};
    use super::*;

    struct FileIterator<R: Read> {
        reader: R,
        buff: Vec<u8>,
    }

    impl<R: Read> LendingIterator for FileIterator<R> {
        type Item<'a> where Self: 'a = &'a [u8];

        fn next(&mut self) -> Option<Self::Item<'_>> {
            let read = self.reader.read(self.buff.as_mut_slice()).unwrap();
            if read <= 0 {
                return None;
            }
            return Some(self.buff.as_slice());
        }
    }

    #[test]
    fn test() {
        let data = Cursor::new("abcdef".as_bytes());
        let it = FileIterator {
            reader: data,
            buff: vec![0u8; 1],
        };


        let mut mapped = it.map_lending(|v| v);

        while let Some(value) = mapped.next() {
            println!("{:?}", value);
        }
    }

    fn do_something(v: &[u8]) -> &[u8] {
        v
    }

}
