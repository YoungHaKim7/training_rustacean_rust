// Defining a trait
//
// Traits are similar to a featue often called interfaces in other languages, although with some differences.
// A "Summary" trait that consists of the behavior provided by a "summarize" method

pub trait Summary {
    fn summary(&self) -> String;
}

fn main() {}
