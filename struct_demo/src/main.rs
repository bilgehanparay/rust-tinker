struct User{
    active: bool,
    username: String, // not &str but String so struct owns its data
    email: String,
    sign_in_count: u64,
}

struct Point {x: i32, y: i32}
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    // area borrows self read-only
    fn area(&self) -> u32{
        self.width * self.height
    }

    // set_width borrows with write permission
    // so it can mutate the struct than return
    // ownership
    fn set_width(&mut self, width: u32){
        self.width = width;
    }

    // max owns both rectangle and other 
    // rectangle than returns another rectangle
    // note that neither self nor other
    // rectangle can be used after calling this
    // function
    fn max(self, other: Rectangle) -> Rectangle{
        Rectangle{
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"), // heap
        username: String::from("someusername123"),  // heap
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = User{
        email: String::from("another@example.com"),
        ..user1 // copy the rest
        // Can't use user1 because string username in user1
        // was in Heap and now user2 owns it!
    };

    let mut p = Point{x: 0, y: 0};
    let x = &mut p.x; // x has read write access to p.x and borrows
    // cannot borrow p or p.x, but can borrow other fields
    *x += 1;
    // borrow ended for p.x

    let rect1 = Rectangle{
        width: 30,
        height: 50
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
/*
//function read-only borrows rectangle
//struct
fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle*height
}
*/
fn build_user(email: String, username: String) -> User{
    User {
        active: true,       // set true
        username,           // received from function parameter
        email,              // received from function parameter
        sign_in_count: 1,   // set one
    }
}
