fn main() {
    let iter = (0..5).into_iter();

    // ... and make a MyCollection out of it
    let c = MyCollection::from_iter(iter);

    println!("{}", c.0);
}
