// The first attempt at defining an enum to represent a cons list data structure of "i32" values

enum List {
    Cons(i32, List)
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
