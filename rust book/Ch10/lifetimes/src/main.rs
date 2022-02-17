struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    // println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // let x : &i32;           // reference
    // let x : &'a i32;        // reference with explicit lifetime
    // let x : &'a mut i32;    // mutable reference with explicit lifetime

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    } // result is valid until here
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(',').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let s: &'static str = "I have a static lifetime.";
}

// // error needs a generic lifetime parameter
// fn longest(x: &str, y: &str) -> &str { // doesn't know the concrete lifetime of the parameters and the return value
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}