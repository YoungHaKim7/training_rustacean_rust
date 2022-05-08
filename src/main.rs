fn lookup_player(id: u32) -> Option<String> {
    if id == 1 {
        return Some("crabby".to_string());
    }

    return None;
}


fn run_game() -> Option<()> {
    // let player = match lookup_player(1) {
    //     Some(p) => p,
    //     None => return
    // };
    let player = lookup_player(1)?;
    println!("Player: {player}");
    Some(())
}

fn main() {
    run_game();
}
