use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is : {}", x);

    // const MAX_POINTS: u32 = 100_000;

    let y = 5;
    let y = y + 1;
    let y = y * 2;

    println!("The value of y is: {}", y);

    let spaces = "   "; // 3 space characters
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    // let mut spaces = "   ";
    // spaces = spaces.len(); // will get a compile error
    // println!("mut spaces: {}", spaces);

    let a = 2.0; // f64
    let b: f32 = 3.0; // f32

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (d, e, g) = tup;
    println!("The value of e is: {}", e);

    let five_hundred = tup.0;
    let siz_point_four = tup.1;
    let one = tup.2;

    let months = ["January", "February", "March", "April", "May", "June", "July",
                    "August", "September", "October", "November", "December"];

    let ar = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Not a number");

    let element = ar[index];

    println!(
        "The value of th element at index {} is: {}", index, element
    );
}
