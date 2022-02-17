fn main() {
    // type specified with <>
    let v: Vec<i32> = Vec::new();

    // infer the type when inserting values
    let v = vec![1, 2, 3]; // macro

    // modifying vectors (mutable)
    let mut v = Vec::new(); // gets type from data

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        let v = vec![1, 2, 3, 4];
    } // vector is dropped when it goes out of scope

    // accessing vector elements
    let v = vec![1, 2, 3, 4];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) { // get() returns Option<&T>
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100]; // panic occurs
    let does_not_exist = v.get(2); // returns None without a panic

    let v = vec![1, 2, 3, 4];
    let first = &v[0];
    // v.push(6); // error - immutable reference in scope, [borrowing rules]
    println!("The first element is {}", first);

    let v = vec![5, 4, 3, 2, 1];
    for i in &v { // i is immutable references
        println!("{}", i);
    }

    let mut v = vec![5, 4, 3, 2, 1]; //mutable vector
    for i in &mut v { // i is mutable references
        *i = *i + 2; // dereference operator *
        println!("{}", i);
    }

    // using enums to store different types in vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}
