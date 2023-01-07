// We are going to simplify the problem to start.
//
//
// What if the input is entirely in memory and only a single `set`?
//

use std::io::BufRead;

pub(crate) fn only_in_memory() {
    // first task is stream data in chunks
    let mut testing = "This, will, be, my test; bytes,".as_bytes();

    // how is it already 30 minutes?!?!?!
    let mut new_buff = Vec::new();
    let result = testing.read_until(b',', &mut new_buff).unwrap();
    testing.read_until(b',', &mut new_buff).unwrap();
    // technically, chunks would be a really nice answer to this but in the real deal, I'll
    // probably have to find a delimiter I think
    while !testing.is_empty() {
        testing.read_until(b',', &mut new_buff).unwrap();
    }
    println!("{:?}", String::from_utf8(new_buff));
}
