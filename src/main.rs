use std::any::Any;
use std::fmt::Debug;
fn do_stuff_depending<T: Any + Debug>(value: &T) {
    // trait object
    log(value);
    println!("R:{}, G:{}, B:{}", RGB.0, RGB.1, RGB.2);
}

fn main() {
    let color = (183, 65, 14);
    do_stuff_depending(&color);

    // let color = Color::HexRGB(12009742);
    //    let color = Color::Named("Rust".to_string());

    // match color {
    //     Color::Named(s) => println!("{s}"),
    //     Color::HexRGB(v) => println!("{v:#x}"),
    // }
}
