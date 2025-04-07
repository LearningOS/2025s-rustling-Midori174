// enums1.rs
//
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Move { x: i32, y: i32},
    Echo(String),
    ChangeColor(u32, u32, u32),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("Hello, world!")));
    println!("{:?}", Message::Move { x: 10, y: 20 });
    println!("{:?}", Message::ChangeColor(255,255,255));
}
