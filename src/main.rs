use std::ops::{Deref, DerefMut};

struct HoldsANumber(u8);

impl HoldsANumber {
    fn prints_the_number_times_two(&self) {
        println!("{}", self.0 * 2);
    }
}

impl Deref for HoldsANumber {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HoldsANumber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let mut my_numuber = HoldsANumber(20);
    *my_numuber = 30;
    println!("{:?}", my_numuber.checked_sub(100));
    my_numuber.prints_the_number_times_two();
}
