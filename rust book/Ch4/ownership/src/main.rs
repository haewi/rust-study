fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); // error

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    println!("x: {}", x);
    // println!("s: {}", s); // error

    let s1 = gives_ownership();
    println!("s1: {}", s1);
    let s2 = String::from("hello");
    println!("s2: {}", s2);
    let s3 = takes_and_gives_back(s2);
    println!("s3: {}", s3);
    // println!("s2: {}", s2); // error

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(s: String){
    println!("from takes_ownership: {}", s);
}

fn makes_copy(x1: i32){
    println!("from makes_copy: {}", x1);
}

fn gives_ownership() -> String{
    let ss = String::from("gives_ownership");
    ss
}

fn takes_and_gives_back(ss: String) -> String{
    ss
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len(); // the length of s
    (s, length)
}