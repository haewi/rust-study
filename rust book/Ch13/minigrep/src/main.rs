use std::env; // to use args() function
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args() // returns a iterator of the command line arguments
                                        .collect(); // turn the iterator into a vector
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1); // stop the program and return the number that was passed as the exit status code
    });
    
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}