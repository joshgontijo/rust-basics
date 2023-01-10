use std::fmt::Debug;
use std::sync::RwLock;

fn main() {}

pub trait FlattenExt: Iterator {
    fn our_flatten(self) -> Flatten<Self>
        where Self: Sized,
              Self::Item: IntoIterator;
}

impl<T> FlattenExt for T where T: Iterator {
    fn our_flatten(self) -> Flatten<Self> where Self::Item: IntoIterator {
        Flatten::new(self)
    }
}


pub struct Flatten<O>
    where O: Iterator,
          O::Item: IntoIterator
{
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
    where O: Iterator,
          O::Item: IntoIterator
{
    fn new(iter: O) -> Self {
        Flatten { outer: iter, inner: None }
    }
}

impl<O> Iterator for Flatten<O>
    where O: Iterator,
          O::Item: IntoIterator
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_it) = self.inner {
                if let Some(i) = inner_it.next() {
                    return Some(i);
                }
                self.inner = None
            }
            let next_inner_iter = self.outer.next()?.into_iter();
            self.inner = Some(next_inner_iter);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    #[test]
    fn test() {
        let mut it = vec![vec!["a"], vec!["b", "c"]].into_iter().our_flatten();

        assert_eq!(Some("a"), it.next());
        assert_eq!(Some("b"), it.next());
        assert_eq!(Some("c"), it.next());
    }
}