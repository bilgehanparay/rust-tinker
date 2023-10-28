
pub fn typecasting_demo(){
    // implicit conversion not allowed
    let decimal = 5.33f32;
    // let integer: u8 = decimal; // wont compile

    // explicit conversion allowed
    // thru as keyword
    let integer = decimal as char;
    let character = integer as char;

    // a float cannot be converted to char
    // let character = decima as char; // wont compile

    // When casting larger ints to smaller ints result is modulus
    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("1000 as a u8 is: {}", 1000 as u8);
    println!("-1 as a u8 is: {}", -1i32 as u8);
    
    
}