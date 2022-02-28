// This new definithon of the "IpAddr" enum says the both "v4" and "v6" variants will have
// associated "String" values.

use std::fmt::Debug;

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn route(ip_kind: IpAddrKind) {}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);
}

