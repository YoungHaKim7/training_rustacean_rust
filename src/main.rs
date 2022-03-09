// Trait part5 - making trait useful with other trait 
use std::fmt::Debug;

struct Monster {
    health: i32,
}

#[derive(Debug)]
struct Wizard {}

#[derive(Debug)]
struct Ranger {}

trait FightClose: Debug{
    fn attack_with_sword(&self, opponent: &mut Monster) where Self: std::fmt::Debug {
        opponent.health -= 10;
        println!(
            "You attack with your sword. Your opponent has {} health left.",
            opponent.health
        );
        println!("You are now in this condition: {:?}", self);
    }

    fn attack_with_hand(&self, opponent: &mut Monster) {
        opponent.health -= 2;
        println!(
            "You attack with your opponent now has {} health left",
            opponent.health
        );
    }
}

impl FightClose for Wizard {}
impl FightClose for Ranger {}

trait FightFromDistance {
    fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
        if distance < 10 {
            opponent.health -= 10;
            println!(
                "You attack with your bow. Your opponent has {} health left",
                opponent.health
            );
        }
    }

    fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
        if distance < 3 {
            opponent.health -= 4;
            println!(
                "You attack with your bow. Your opponent has {} health left",
                opponent.health
            );
        }
    }
}

impl FightFromDistance for Ranger {}


fn main() {
    let radagast = Wizard {};
    let aragorn = Ranger {};

    let mut uruk_hai = Monster {health: 40};

    let distance = 8;
    radagast.attack_with_sword(&mut uruk_hai);
    aragorn.attack_with_bow(&mut uruk_hai, distance);
}
