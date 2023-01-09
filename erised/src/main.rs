mod io;
use io::{from_file, only_in_memory};

fn main() {
    only_in_memory();
    from_file();
}
