fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear(); // makes it an empty String ""
    // word is 5, but can't use it because s is now ""

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}, {}", hello, world);

    let hello = &s[..5]; // from start
    let world = &s[6..]; // till last
    let world2 = &s[6..s.len()]; // till last

    println!("{}, {} - {}", hello, world, world2);

    let s = String::from("hello world");
    let word = first_word_v2(&s);
    println!("word: {}", word);

    let s = String::from("hello world");
    let word = first_word_v3(&s[..]);
    println!("word: {}", word);
}

fn first_word(s: &String) ->  usize {
    let bytes = s.as_bytes();

    // i: index, item: reference to the element
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // byte literal syntax
            return i;
        }
    }

    s.len()
}

fn first_word_v2(s: &String) -> &str {  // &str is a "string slice" type
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn first_word_v3(s: &str) -> &str { // can be used with both &str and &String values
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}