use std::env;
use std::process;

use minigrep::Config;

fn main() {
    /*
    TODO: use args_os instead of args to handle invalid Unicode strings
    */
    let args: Vec<String> = env::args().collect();

    // env::args returns an iterator and we pass the ownership
    // of the iterator to function
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config){
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
