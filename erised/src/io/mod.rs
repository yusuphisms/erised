use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// What if the input is from a file?
/// New line delimited
pub(crate) fn from_file() {
    let file_handle = File::open("/home/mkuki/stream/erised/erised/src/io/testfile").unwrap();
    // In the real deal, we'd have to make many chunky reads
    // While we can't store everything in memory, striking a balance between many
    // network calls and memory is worth considering. BufReader and similar implementations could
    // help here.
    let mut buf_reader = BufReader::new(file_handle);
    // how is it already 30 minutes?!?!?!
    let mut new_buff = Vec::new();
    let result = buf_reader.read_until(b'\n', &mut new_buff).unwrap();
    println!("{:?}", String::from_utf8(new_buff.clone()));
    while !buf_reader.buffer().is_empty() {
        buf_reader.read_until(b'\n', &mut new_buff).unwrap();
    }
    println!("{:?}", String::from_utf8(new_buff));
}

/// We are going to simplify the problem to start.
/// What if the input is entirely in memory and only a single `set`?
pub(crate) fn only_in_memory() {
    // first task is stream data in chunks
    let mut testing = "This, will\n be, my test; bytes,".as_bytes();

    // how is it already 30 minutes?!?!?!
    let mut new_buff = Vec::new();
    let result = testing.read_until(b'\n', &mut new_buff).unwrap();
    println!("{:?}", String::from_utf8(new_buff.clone()));
    //testing.read_until(b',', &mut new_buff).unwrap();
    // technically, chunks would be a really nice answer to this but in the real deal, I'll
    // probably have to find a delimiter I think
    while !testing.is_empty() {
        testing.read_until(b'\n', &mut new_buff).unwrap();
    }
    println!("{:?}", String::from_utf8(new_buff));
}
