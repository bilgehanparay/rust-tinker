use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

/*
This directive tells the serde crate to examine the 
type when the code is compiled and automatically generate code
to parse a value of this type from data in format of HTML form
in POST request
*/
#[derive(Deserialize)]
struct GcdParameters{
    n: u64,
    m: u64,
}

fn main(){
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Serving on http://localhost:2500...");
    server
        .bind("127.0.0.1:2500").expect("error binding server to address")
        .run().expect("error running server");
}

fn get_index() -> HttpResponse{
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,
        )
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse{
    if form.n == 0 || form.m == 0{
        return HttpResponse::BadRequest()
                .content_type("type/html")
                .body("Computing the GCD with zero is boring.");
    }
    let response = format!("The gcd of numbers {} and {} is <b>{}</b>\n",
                            form.n, form.m, gcd(form.n, form.m));
    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}
/*
fn main_gets_args() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..]{
        d = gcd(d, *m);
    }
    println!("The greates common divisor of {:?} is {}", numbers, d)
}
*/
fn gcd(mut n: u64, mut m: u64) -> u64{
    assert!(n != 0 && m!= 0);
    while m != 0 {
        if m<n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd(){
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}