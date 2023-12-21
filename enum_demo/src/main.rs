enum IpAddr{
    V4(String),
    V6(String),
}
/* Also possible with enum:
enum IpAddr{
    V4(u8, u8, u8, u8),
    V6(String),
}
*/

enum Message{
    Quit, 
    Move{ x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enum can be implemented
impl Message{
    fn call(&self){
        match self{
            Message::Write(msg) => { println!("Write: {}", msg); }
            Message::Move{x,y}  => { println!("Move: {} {}", x, y); }
            Message::Quit    => { println!("Quit:"); }
            Message::ChangeColor(r,g,b) => { println!("ChangeColor: {} {} {}", r, g, b); }
        }

    }

}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    let m = Message::Write(String::from("hello"));
    m.call();
}


/*
In other words, you can think of if let 
as syntax sugar for a match that runs 
code when the value matches one pattern 
and then ignores all other values.
*/