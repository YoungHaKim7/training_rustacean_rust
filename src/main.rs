// Catch-all Patterns and the _ Placeholder
//
fn add_fancy_hat() {
    println!("add_fancy_hat!!!function!!")
}
fn remove_fancy_hat() {
    println!("remove_fancy_hat!!!function!!")
}
fn move_player(num_spaces: u8) {
    println!("move_player__!!!function!!")
}

fn main() {

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

}
