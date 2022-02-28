// Defining an Enum
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {}

fn main() {
    // We can call this function with either variant:

    println!("{:?}", route(IpAddrKind::V4));
    println!("{:?}", route(IpAddrKind::V6));
}

