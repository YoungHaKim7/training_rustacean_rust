// Module system
//
// Packages
// Crates
// Modules and use
// Paths
// Bringing "HashMap" into scope in an idiomatic way
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
}
