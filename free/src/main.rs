
fn main() {
    println!("Hello, world!");
}

/* signed unsigned integers
i8, i16, i32,
i64, i128, u8, 
u16, u32, u64,
u128
*/

/* signed and unsigned integers 
the same size as an address on 
the machine
isize, usize
*/

/* single point/double point float
f32, f64
*/

/*boolean
bool*/

/*unicode character 32bits
char*/

/* mixed type tuple
(char, u8, i32)
*/

/*Unit-empty tuple
()*/

/* named field struct
stuct S{
    x: f32,
    y: f32
}
*/

/* tuple-like struct
struct T(i32, char)
*/

/*unit like struct
struct E*/

/*enum algebraic data type
enum attend {OnTime, Late(u32)}*/

/*Box: owning pointer to value in heap
Box<Attend>*/

/*utf-8 string dynamically sized
String*/

/*reference to str: non-owning pointer to utf-8 text
&str*/

/*Fixed length array of elements fixed type
[f64; 4], [u8; 256]*/

/*Vector, varying length, elements all same type
Vec<f64>*/

/*Reference to slice: reference to a portion of an array
or vector, comprising pointer and length
&[u8], &mut [u8]
&v[10..20]
&mut a[..]*/

/*Optional value: either None or Some(v) present with value v
Option<&str>
Some("Dr"), None*/

/*Result of aoperation that my fail either a success value
Ok(v), or an error Err(e)
Result<u64, Error>*/

/*Trait object, reference to any value that implemets a given 
set of methods
&dyn Any, &mut dyn Read*/

/*Pointer to function
fn(&str) -> bool
str::is_empty*/

/*Clousure -> like lambda functions?
|a,b|{a*a + b*b}*/


















