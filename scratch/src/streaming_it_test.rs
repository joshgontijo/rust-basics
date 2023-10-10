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
        let mut it = FileIterator {
            reader: data,
            buff: vec![0u8; 1],
        };


        let mut mapped = it.map_lending(do_something);

        while let Some(value) = mapped.next() {
            println!("{:?}", value);
        }
    }

    fn do_something(v: &[u8]) -> &[u8] {
        v
    }

}