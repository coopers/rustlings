// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

// 

#[derive(Debug)]
enum Message {
    Echo(String),
    ChangeColor(u8, u8, u8),
    Move { x: i32, y: i32 },
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Move { x: 10, y: 30 },
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
