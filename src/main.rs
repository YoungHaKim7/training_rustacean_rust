fn append_string(buffer: &mut Vec<u8>, data: &str) {
    for value in data.bytes() {
        buffer.push(value)
    }
}

fn main() {
    let mut buffer = Vec::new();
    append_string(&mut buffer, "안녕 ");

    let result = String::from_utf8(buffer).unwrap();
    let u8_result = result.bytes();
    println!("{result:?}");
    println!("{u8_result:?}");
}
