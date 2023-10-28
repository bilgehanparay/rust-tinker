/*
Custom types can be defined
- structs
- enum
*/

/*
Structs
*/
// tuple Structs
struct Pair(i32, f32);
// c-type structs
[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
// unit struct
struct Unit;

struct Point{
    x: f32,
    y: f32,
}

// struct inside struct
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

pub struct_demo(){
    // make a new struct from fields of other struct
    let point:Point = Point{x:2.2, y:3.3};
    let a_point = Point{x: 5.2, ..point};
    
    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;
}