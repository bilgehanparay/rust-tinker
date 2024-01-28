
fn main() {
    /*
        'string' parameters holds the pointer 
        to the first character of the string
    */
    let string = "Hello World";
    copy_trait_demo();
    mutable_reference_demo();
    let mut s = Box::new(42);
    replace_with_84(&mut s);
    println!("{s}");
}

fn copy_trait_demo(){
    /*
        Ownership example: 'Copy trait'
    */
    // Defaults to i32, which implments copy trait
    let x1 = 42;
    // Allocates memory on the heap and does not implement copy trait
    let y1 = Box::new(84);

    // Create a new scope
    {
        // z now has the ownership of x1 and y1
        // but x1 is copied but y1 is moved!
        let z = (x1, y1);
    }
    // z goes out of scope so all memory it owned are freed
    // We can do this, since x1 was copied when we transferred it to z
    let x2 = x1;
    // This wont compile, because y1 was freed when z went out of scope
    //let y2 = y1; // -
}

fn mutable_reference_demo(){
    /*
        Mutable references
    */
    // xx is immutable, you cannot change xx 
    let xx = 42;
    // y is a mutable reference: you can change y to refer 
    // another variable; but you cannot change the value y
    // refers to
    let mut y = &xx;

    // x is immutable reference: you can't change it to refer 
    // another variable but you can change the value that z refers
    let z = &mut y;
    let mut z2 = 5;
    // This is valid: z is itself immutable but the value z refers
    // is a mutable reference.y now refers to parameter z2
    *z = &z2; 
}

fn replace_with_84(s: &mut Box<i32>){
    // s: is an immutable reference that refers to a 
    // mutable Box<i32> object

    // This does not compile,
    // was assumes the ownership of Box<i32> that
    // s refers, but caller did not give the ownership
    // let was = *s;
    // see the ref: https://doc.rust-lang.org/std/mem/fn.take.html
    let was = std::mem::take(s);
    // This is valid since *s is a mutable Box<i32>
    *s = was;
    let mut r = Box::new(84);
    std::mem::swap(s, &mut r);
    assert_ne!(*r, 84);
}

/* 
    Multiple lifetimes
*/
// Should use multiple lifetimes when there is a data structure
// with multiple references, and function returning new references
struct StrSplit<'s, 'p>{
    delimiter: &'p str,
    document: &'s str,
}
impl<'s, 'p> Iterator for StrSplit<'s, 'p>{
    type Item = &'s str; // We use lifetime of 's, 
    // since we get a reference into the document
    fn next(&self) -> Option<Self::Item>{
        // todo:
    }
}
fn str_before(s: &str, c: char) -> Option<&str>{
    // Notice that, the lifetime of the delimiter 
    // is local to the function StrSplit
    StrSplit {document: s, delimiter: &c.to_string() }.next()
}
//****************************************************************************
/*
    Alignment/Layout
*/
// Type Foo will be aligned as if C compiler would lay out this struct
// So struct is aligned as follows:
// tiny   -1 bit - takes up 4 bytes(3 byte padding, 1 byte the value)
// normal -4 bytes-takes up 4 bytes, no padding
// small  -1 byte-takes up 8 bytes, 7 bytes padded to align long
// long   -8 bytes-takes up 8bytes, no bytes padded
// short  -2bytes-takes up 8bytes, 6 bytes padded
// ---    -to align Foo with 32bytes 6 bytes padded
// 32 bytes
#[repr(C)] 
struct Foo{
    tiny: bool, //1- byte aligned
    normal: u32,//4 byte aligned
    small: u8, // 1 byte aligned
    long: u64, //8 byte aligned
    short: u16,// 2 byte aligned
}
// but this is now takes up 16bytes
// since Foo2 is aligned. 
// repr(rust) allows compiler to rearrange fields!
struct Foo2{
    long: u64,
    normal:u32,
    short: u16,
    small: u8,
    tiny: bool
}
// repr(packed) -> disable padding to reduce memory usage
// but code might be slower due to misalinged acces or crash
// if we call a cpu function that only supported in aligned access
//****************************************************************************