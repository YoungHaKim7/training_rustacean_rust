// A shorcut for Propagating Errors: the ? Operator
// Using "fs::read_to_string" instead of opening and then reading the file

use std::fs; 
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {}
