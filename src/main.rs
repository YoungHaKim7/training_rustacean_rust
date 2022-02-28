// Here's method named "call" that we could define on our "Message" enum

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// unit struct
struct QuitMessage;

struct MoveMessage {
    x: i32,
    y: i32,
}

// tuple struct
struct WriteMessage(String);

// tuple struct
struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
fn main() {
    let m = Message::Write(String::from("Hello"));

    m.call();
}
