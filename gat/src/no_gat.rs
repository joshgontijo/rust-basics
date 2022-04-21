trait FnMut1Arg<A>: FnMut(A) -> <Self as FnMut1Arg<A>>::Output {
    type Output;
}
impl<F: ?Sized, A, O> FnMut1Arg<A> for F
    where
        F: FnMut(A) -> O,
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
    let mut wrapped_item = iterator.map(|e| Wrapper { e });
    while let Some(item) = wrapped_item.next() {
        println!("{:?}", item);
    }
}

#[derive(Debug)]
struct Wrapper<'a, T: ?Sized> {
    e: &'a T,
}

struct FileIt;

impl<'n> HasItem<'n> for FileIt {
    type Item = &'n [u8];
}
impl GatIterator for FileIt {
    fn next(&mut self) -> Option<Item<'_, Self>> {
        todo!()
    }
}

trait HasItem<'a, __ = &'a Self> {
    type Item;
}
type Item<'a, This> = <This as HasItem<'a>>::Item;
trait GatIterator: for<'n> HasItem<'n> {
    fn next(&mut self) -> Option<Item<'_, Self>>;

    fn map<F>(self, f: F) -> Map<Self, F>
        where
            Self: Sized,
            for<'n> F: FnMut1Arg<Item<'n, Self>>,
    {
        Map { it: self, f }
    }
}

#[derive(Debug)]
pub struct Map<I, F> {
    it: I,
    f: F,
}

impl<'n, I, F> HasItem<'n> for Map<I, F>
    where
        I: GatIterator,
        F: FnMut1Arg<Item<'n, I>>,
{
    type Item = <F as FnMut1Arg<Item<'n, I>>>::Output;
}

impl<I, F> GatIterator for Map<I, F>
    where
        I: GatIterator,
        for<'n> F: FnMut1Arg<Item<'n, I>>,
{
    fn next(&mut self) -> Option<Item<'_, Self>> {
        self.it.next().map(&mut self.f)
    }
}