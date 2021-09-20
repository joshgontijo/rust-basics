#![feature(generic_associated_types)]

trait FnMut1Arg<A>: FnMut(A) -> <Self as FnMut1Arg<A>>::Output {
    type Output;
}

impl<F: ?Sized, A, O> FnMut1Arg<A> for F
    where F: FnMut(A) -> O,
{
    type Output = O;
}

fn main() {
    let iterator = FileIt;

    // doesn’t work because type inference for closures is dumb…
    // let mut wrapped_item = iterator.map(|e| Wrapper { e });

    // let’s help inference out
    fn help_inference_out<F: FnMut(&[u8]) -> Wrapper<'_, [u8]>>(f: F) -> F {
        f
    }
    let mut wrapped_item = iterator.map(help_inference_out(|e| Wrapper { e }));
    while let Some(item) = wrapped_item.next() {
        println!("{:?}", item);
    }
}

#[derive(Debug)]
struct Wrapper<'a, T: ?Sized> {
    e: &'a T,
}

struct FileIt;

impl GatIterator for FileIt {
    type Item<'n> where Self: 'n = &'n [u8];

    fn next(&mut self) -> Option<Self::Item<'_>> {
        todo!()
    }
}

trait GatIterator {
    type Item<'n> where Self: 'n;

    fn next(&mut self) -> Option<Self::Item<'_>>;

    fn map<F>(self, f: F) -> Map<Self, F>
        where
            Self: Sized,
            for<'n> F: FnMut1Arg<Self::Item<'n>>,
    {
        Map { it: self, f }
    }
}

#[derive(Debug)]
pub struct Map<I, F> {
    it: I,
    f: F,
}

impl<'a, I, F> GatIterator for Map<I, F>
    where
        I: GatIterator,
        for<'n> F: FnMut1Arg<I::Item<'n>>,
{
    type Item<'n>
        where
            Self: 'n,
    = <F as FnMut1Arg<I::Item<'n>>>::Output;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.it.next().map(&mut self.f)
    }
}