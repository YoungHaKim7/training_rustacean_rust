#[derive(Debug)]
enum SomeEnum<T> {
    OptionA(T),
    OptionB(T),
    OptionC,
}

fn main() {
    let some_data = SomeEnum::OptionA(34.2);

    match some_data {
        SomeEnum::OptionA(a) => println!("OptionA contains {}", a),
        SomeEnum::OptionB(b) => println!("OptionB contains {}", b),
        SomeEnum::OptionC => println!("Boring option c : "),
    }

    let some_data2 = SomeEnum::OptionB('b');
    println!("{some_data2:?}");
    let some_data3 = SomeEnum::OptionA(vec![1, 2, 3]);
    println!("{some_data3:?}");
}
