fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    println!("s: {}", s);
    let s = "initial contents".to_string();
    println!("s: {}", s);

    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
    s.push('l'); // push a single character
    println!("{}", s);

    // add two strings to one
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // s1 can't be used (moved)
    println!("s2: {} = s3: {}", s2, s3);

    // combine with format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // doesn't take ownership
    println!("{}", s);

    // error - can't access string elements with index
    let s1 = String::from("hello");
    // let h = s1[0];

    // slicing Strings are fine
    let hello = String::from("Здравствуйте");
    let s = &hello[0..4]; // 4 bytes = 2 bytes + 2 bytes
    println!("{} {}", hello, s);

    // iterating
    for c in "नमस्ते".chars() { // Rust's char type
        println!("{}", c);
    }
    for c in "नमस्ते".bytes() { // raw byte
        println!("{}", c);
    }
}
