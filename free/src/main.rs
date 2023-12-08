use std::fmt; 

mod arrays;

#[derive(Debug)]
struct Complex{
    img:  f64,
    real: f64,
}
// İmplement this function so that
// when println! the defined struct
// it is printed properly
impl fmt::Display for Complex{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.img)
    }
}

struct List(Vec<i32>);
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}
impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
       let lon_c = if self.lon >= 0.0 {'E'} else {'W'};
       write!(f, "{}: {:.3}°{} {:.3}°{}", 
                self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

struct Color{
    red: u8,
    green: u8,
    blue: u8
}
impl fmt::Display for Color{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({red}, {green}, {blue}, 0x{red:0>2X}{green:0>2X}{blue:0>2X}", 
            red=self.red, green=self.green, blue=self.blue)
    }
}

// function can take tuple argument and 
// return tuple argument
fn reverse(pair: (i32, bool)) -> (bool, i32){
    let (int_param, bool_param) = pair; // binds parameters of tuple
    (bool_param, int_param)
}

#[derive(Debug)]
struct Matrix{
    a11: f64,
    a12: f64,
    a21: f64,
    a22: f64,
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}\n{} {}", self.a11, self.a12, self.a21, self.a22)
    }
}

fn main() {
    println!("{} days", 0xFF);
    println!("{} days", 0b0011);
    println!("{0}, this is {1}. {1}, this is {0}", 
              0xFFF, 0xAF);
    println!("{subject} {verb} {object}", 
            subject="a subject",
            verb="a verb",
            object="an object");
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 10:               {}",   69420+0xFF);
    println!("{number:.>5}", number=23);
    println!("{number:0>width$}", number=23, width=5); // 5->width parameterized
    let pi = 3.123456;
    println!("pi is {:.2}", pi);
    let c1 = Complex{img: 2.134, real: 45.6};
    println!("Debug: {:?}", c1);
    println!("Display: {}", c1);
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ] {
        println!("{}", city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{}", color);
    }

    // RUST primitive data types
    let logical: bool = true; // variable name is annotated
    let a_float: f64 = 1.0; // regular annotation
    let a_int = 5i32; // values can be suffix annotated
    
    /*
    a mutable variable can change value but not type
    */
    let mut mutable = 12;
    mutable = 21; // OK
    // mutable = true; // Won't Compile
    let mutable = true; // this is OK

    /*
    Underscores in numbers are OK
    */
    let ii = 1_000_000; // same as 1000000
    let ef64 = 2.6e-6; // this is e-notation, OK, f64 type

    /*
    RUST Tuples
    */
    // A tuple can take different types
    let some_tuple = (1u8, 2u16, true, -3i32);
    // indexing rust tuples
    println!("some tuple first value: {}", some_tuple.0);
    println!("some tuple first value: {}", some_tuple.1);
    // tuple inside tuple
    let tuple_tuple = ((1u8, 2.3f64, true),(6u8, -3i32));
    println!("tuple of tuples {:?}", tuple_tuple);
    let m = Matrix{a11: 1.1, a12:1.2, a21:1.3, a22:1.4};
    println!("{}", m);
    println!("{:?}", m);
    arrays::array_fn();
    enums::enum_demo();
}














