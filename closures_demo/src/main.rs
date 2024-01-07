fn main() {
    // Immutable borrow
     let list = vec![1, 2, 3];
     println!("Before defining closure: {:?}", list);

     let only_borrows = || println!("From closure: {:?}", list);

     println!("Before calling closure: {:?}", list);
     only_borrows();
     println!("After calling closure: {:?}", list);

     // Mutable borrow
     let mut list2 = vec![1, 2, 3];
     println!("Before defining closure: {:?}", list2);

     let mut borrows_mutably = || list2.push(7);
     // Wont Compile -> println!("Can I call this: {:?}", list2);
     borrows_mutably();
     println!("After calling closure: {:?}", list2);

     // pass owneership to closure
     let list3 = vec![1, 2, 3];
     println!("Before defining closure: {:?}", list);

     thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}