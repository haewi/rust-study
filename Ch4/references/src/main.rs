fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // change(&s1); // error cannot change something borrowed

    let mut s2 = String::from("hello");
    change2(&mut s2);
    println!("s2: {}", s2);

    let mut s2 = String::from("hello");

    let r1 = &mut s2;
    // let r2 = &mut s2; // error: can't have two mutable reference

    // println!("{}, {}", r1, r2); // error

    {
        let r1 = &mut s2;
    }
    let r2 = &mut s2; // no error, different scope


    let mut s = String::from("hello");

    let r1 = &s; // fine
    let r2 = &s; // fine
    // let r3 = &mut s; // not fine, can't have both mutable reference and immutable reference at the same time

    // println!("{}, {}, and {}", r1, r2, r3);



    let mut s = String::from("hello");

    let r1 = &s; // fine
    let r2 = &s; // fine
    println!("{} and {}", r1, r2); // r1, r2 is not used after, so the references's scope is over

    let r3 = &mut s; // so this is fine
    println!("{}", r3); // no error

    // dangling reference
//     let reference_to_nothing = dangle(); // dangling reference returned
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn change(s: &String) {
//     s.push_str(", world!"); // references are also immutable
// }

fn change2(s: &mut String) {
    s.push_str(", world!");
}

// fn dangle() -> &String{
//     let s = String::from("hello");

//     &s // return just s directly and there will be no error
// } // s out of scope, s is dropped