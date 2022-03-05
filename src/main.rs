// A shorcut for Propagating Errors: the ? Operator
// Where the "?" Operator Can Be Used

use std::fs::File; 

fn main() {
    let f = File::open("hello.txt")?;
}
