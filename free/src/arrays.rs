/*
Arrays and slices
*/

// borrows a slice(from array)
fn analyze_slice(slice: &[i32]){
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

pub fn array_fn(){
    // Arrays are similar to C arrays
    // array lengths are known at compile time
    let xs: [i32; 2] = [1, 2];
    // all array elements inited to same value
    let ys: [i32; 500] = [0; 500]; // all zero
    // index and len of array
    println!("xs[0]: {}", xs[0]);
    println!("xs[0]: {}", ys[0]);
    println!("len ys: {}", ys.len());

    // borrow entire section of array
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1 .. 4]); //index 1,2,3

    // how to safely acces arrays
    // use get, match the output to Some/None
    for i in 0..xs.len()+1{ // +1 would couse runtime error index out of bound
        match xs.get(i){
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("too far {}", i)
        }
    }
}