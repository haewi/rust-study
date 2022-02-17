use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
    // returns io::Error struct in Err when failed
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) =>file,
        Err(error) => match error.kind() {
            // if error occured because the file doesn't exist
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            }
            // other errors
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // will retrun value inside Ok or call panic!
    let f = File::open("hello.txt").unwrap();

    // same as unwrap but lets us write the panic! message
    let f = File::open("hello.txt").expect("Failed to open hello.txt");


}

fn read_username_from_file1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // ? operator after the Result enum work almost the same as match
    let mut s = String::new();
    f.read_to_string(&mut s)?; // ? operator converts the error type to the defined return type of this function
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}


fn read_username_from_file4() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}
