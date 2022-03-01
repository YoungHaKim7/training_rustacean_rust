// we can express that by using the unit value(the empty tuple type we mentioned in "The Tuple Type" section) as the code that goes with the "_" arm:
//
fn add_fancy_hat() {
    println!("add_fancy_hat!!!function!!")
}
fn remove_fancy_hat() {
    println!("remove_fancy_hat!!!function!!")
}

fn main() {

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

}
