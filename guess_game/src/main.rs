use std::io;

fn main() {
    println!("Guess the n!");
    println!("input n");

    let mut g = String::new();

    io::stdin()
        .read_line(&mut g)
        .expect("Failed");
    println!("guessed: {g}");

    let ap = 5;
    println!("ap {ap}");
    {
        let mut apb = &ap;
        &apb = 6;
        println!("apb {apb}");
    }
    println!("ap {ap}");
}
