// Generic Types, Traits.
// Code to find the largest number in a list of numbers
//

fn main() {
    let number_list = vec![34, 50, 25, 100, 65, 90];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
}
