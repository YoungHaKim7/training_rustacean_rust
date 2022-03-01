// If we wanted to count all non-quarter coins we see while also announcing the state of the quarters, we could do that with a "match" expression like this:
// same code

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}



fn main() {

    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
