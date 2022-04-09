// Generics

// enum Option<T> {
//     Some(T)
//     None,
// }

// enum Option<i32> {
//     Some(i32)
//     None,
// }

fn option_example(x: i32) -> Option<String> {
    match x > 2 {
        true => Some(String::from("result")),
        false => None,
    }
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn error_handling_example_1(dir: &str) {
    println!("\n\n");

    let mut list_cmd = std::process::Command::new("ls");

    match list_cmd.current_dir(dir).status() {
        Ok(cmd) => cmd,
        Err(e) => panic!("Error: {e}"),
    };

    println!("\n\n");
}

enum Custom<T, U> {
    EXAMPLE(T),
    SAMPLE(U),
}

// Struct

#[derive(Debug)]
struct Rectangle<T> {
    height: T,
    width: T,
}

#[derive(Debug)]
struct Cube<T, U, V> {
    height: T,
    width: U,
    length: V,
}

fn main() {
    let rect1 = Rectangle {
        height: 1,
        width: 2,
    };
    let rect2 = Rectangle {
        height: 1.65,
        width: 2.22,
    };

    let cube1 = Cube {
        height: 1,
        width: 2,
        length: 3,
    };
    let cube2 = Cube {
        height: 1.54,
        width: 2,
        length: 3.75,
    };

    println!("Rect1 = {rect1:?}");
}
