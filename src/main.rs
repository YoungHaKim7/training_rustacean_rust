// Changing "main" to return "Result<(), E>" allow the use of the "?" operator on "Result" values

use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
