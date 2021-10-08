use std::io::{BufRead, Write};
use std::fs::File;
use streaming_iterator;
use streaming_iterator::StreamingIterator;
use std::io;
use std::ops::Add;

pub fn run() {
    let mut data = vec![];
    for i in 0..5 {
        data.write(bincode::serialize(&String::from("#")
            .add(i.to_string().as_str())
            .add("\n"))
            .unwrap().as_slice())
            .unwrap();
    }
    let reader = io::Cursor::new(data.as_slice());


    let mut lines = Lines { reader, buffer: "".to_string(), next: None };

    let res = lines.map(|v| bincode::deserialize(v).unwrap())
        .for_each(|v: &String| {
            println!("{:?}", v);
        });
}


struct FileIt<'it> {
    handle: &'it File,
    buf: &'it mut [u8],
    file_pos: u64,
    buf_pos: usize,
}


#[derive(Debug)]
struct Res<'a> {
    data: &'a str,
    size: usize,
}

struct Lines<B: BufRead> {
    reader: B,
    buffer: String,
    next: Option<Next>,
}

struct Next {
    offset: usize,
    len: usize,
}


impl<B: BufRead> StreamingIterator for Lines<B> {
    // Streaming iterator: we borrow from `self`
    type Item = [u8];

    fn advance(&mut self) {
        self.buffer.clear();
        match self.reader.read_line(&mut self.buffer) {
            Ok(read) => {
                if read == 0 {
                    self.buffer.clear();
                    return;
                }
            }
            Err(e) => panic!("{}", e)
        }
    }

    fn get(&self) -> Option<&Self::Item> {
        let slice = self.buffer.as_str();
        if slice.is_empty() { None } else { Some(slice.as_bytes()) }
    }
}