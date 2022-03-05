// A shorcut for Propagating Errors: the ? Operator
// Where the "?" Operator Can Be Used
// Using the "?" Operator on an "Option<T>" value

use std::fs::File; 

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {}
