struct Monster {
    health: i32
}

struct Wizard {}
struct Ranger {}

trait FightClose {
    fn attack_with_sword(&self, opponent: &mut Monster) {
        opponent.health -= 10;
        println!("You attack with your sword. Your opponent has {} health left.", opponent.health);
    }
}


fn main () {}
