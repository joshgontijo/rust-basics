use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Memtable {
    data: Box<[u8]>,
    offset: AtomicUsize
}

impl Memtable {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![0u8; capacity].into_boxed_slice(),
            offset: AtomicUsize::new(0),
        }
    }

    pub fn append(&self, event: &[u8]) -> usize {
        let offset = self.offset.fetch_add(event.len(), Ordering::SeqCst);
        unsafe {
            let ptr = self.data[offset..offset + event.len()].as_ptr() as *mut u8;
            ptr::copy_nonoverlapping(event.as_ptr(), ptr, event.len());
        }
        offset
    }

    pub fn read(&self, offset: usize, size: usize) -> Vec<u8> {
        self.data[offset..offset + size].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut memtable = Memtable::new(100);

        let data  = "hello".as_bytes();

        println!("Append");
        let offset = memtable.append(data);
        println!("Read");
        let res = memtable.read(offset, data.len());
        assert_eq!("hello", String::from_utf8(res).unwrap().as_str());


    }
}