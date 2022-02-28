// Defining an Enum
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    // We can create instances of each of the tow variants of "IpAddrKind" 
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{four:?}");
    println!("{six:?}");
}

