fn main() {
    println!("Hello, world!");
}

struct MyIt<'a, T> {
    items: &'a [T]
}

impl<'a, T> MyIt<'a, T> {
    fn new(elem: &'a [T]) -> Self {
        MyIt {
            items: elem
        }
    }
}

impl<'a, T> Iterator for MyIt<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.items.is_empty() {
            return Option::None;
        }
        let element = self.items.get(0);
        self.items = if self.items.len() == 1 { &[] } else { &self.items[1..] };
        return element;
    }
}

//------------- MUT
struct MyMutIt<'a, T> {
    items: &'a mut [T]
}

impl<'a, T> MyMutIt<'a, T> {
    fn new(elem: &'a mut [T]) -> Self {
        MyMutIt {
            items: elem
        }
    }
}

impl<'a, T> Iterator for MyMutIt<'a, T> {
    type Item = &'a mut T;

    //'next here is not really needed
    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        //temporarily replaces the field with a an empty array
        //replace will return the 'items' that has the 'a lifetime
        let slice_ref = &mut self.items; //self.items has 'next lifetime
        let a_scoped_slice = std::mem::replace(slice_ref, &mut []);

        let (first, rest) = a_scoped_slice.split_first_mut()?;
        self.items = rest;
        Some(first)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_immutable() {
        let array = [1, 2, 3];
        let it = MyIt::new(&array);

        for (idx, elem) in it.enumerate() {
            assert_eq!(&array[idx], elem);
        }
    }

    #[test]
    fn test_mutable() {
        let mut array = [1, 2, 3];
        let it = MyMutIt::new(&mut array);

        for (_, elem) in it.enumerate() {
            *elem = *elem + 1;
        }

        assert_eq!(Some(&2), array.get(0));
        assert_eq!(Some(&3), array.get(1));
        assert_eq!(Some(&4), array.get(2));

    }
}
