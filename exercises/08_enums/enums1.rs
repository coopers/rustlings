// enums1.rs
//
// No hints this time! ;)

// 

#[derive(Debug)]
enum Message {
    Echo,
    Move,
    Quit,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::ChangeColor);
}
