use std::io::{Cursor, Read, Seek};
use std::ops::DerefMut;
use std::rc::Rc;

fn main() {
    let data = Cursor::new("abcdef".as_bytes());

    let it = FileIterator {
        reader: data,
        buff: Rc::new(vec![0u8; 1].into_boxed_slice()),
    };


    // let mut values = vec![]; //uncomment to allocate a new Rc every iteration
    for entry in it {
        let value = String::from_utf8_lossy(entry.as_ref());
        println!("{:?}", value);
        // values.push(entry); //uncomment to allocate a new Rc every iteration
    }
}

struct FileIterator<R: Read> {
    reader: R,
    buff: Rc<Box<[u8]>>,
}

//A simple and easy way to get around the 'Streaming iterator' problem with Rust
//This allows to return a Rc<T> which gives flexibility on the consumer side,
//If the entry is hold for longer than the lifetime of `next` then a new instance of that type is initialised
impl<R> Iterator for FileIterator<R> where R: Read {
    type Item = Rc<Box<[u8]>>;

    fn next(&mut self) -> Option<Self::Item> {
        let rc = Rc::get_mut(&mut self.buff);
        let res = if rc.is_none() {
            self.buff = Rc::new(vec![0u8; 1].into_boxed_slice());
            Rc::get_mut(&mut self.buff).unwrap()
        } else {
            rc.unwrap()
        };


        let read = self.reader.read(res.deref_mut()).unwrap();
        if read <= 0 {
            return None;
        }
        Some(self.buff.clone())
    }
}

