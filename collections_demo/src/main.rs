use::collections::HashMap;

fn main() {
    // mutating a vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    // reading from vector
    let mut v1 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v1[2];
    println!("The third element is {third}");

    // wont compile
    v1.push(11);
    let third: Option<&i32> = v.get(6);
    match third{
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    
    /*
    Borrowing rules prevents compilation if 
    let first .. row is above v.push(5)
    */
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(5);
    let first = &v[0];
    println!("The first element is: {first}");

    /*Immutable reference to iterate over vector*/
    let vim = vec![100, 32, 57];
    for n_ref in &v{
        let n_plus_one: i32 = *n_ref + 1;
        println!("{n_plus_one}");
    }

    // Strings (as Collections)
    let mut s= String::new();
    let data = "initial contents";
    
    // String is different than string literal
    let s2 = "initial contents".to_string();

    // Appending to a String
    let mut s3 = String::from("foo");
    // push a string literal at the end of string
    s3.push_str("bar");
    // push a single character
    s3.push('l');

    // concatenate using +
    let s11 = String::from("Hello, ");
    let s22 = String::from("World!");
    // s11 is moved and s22 is read only borrowed
    // cannot use s1 after this but can use s2
    // + operator takes ownship of s11 and 
    // copies contents of s22 into s11
    // and returns ownship of s11 to s33
    // s11 can no longer be used after this
    let s33 = s11 + &s22;

    // indexing strings
    // String cannot be indexed with [0]
    // to iterate over characters, use .chars()
    for c in "Bu Bir UTF-8 string şççöğü".chars(){
        println!("{c}");
    }

    // to iterate over bytes, use .bytes()
    for b in "Bu Bir UTF-8 string şççöğü".bytes(){
        println!("{b}");
    }

    // hashmaps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
