use std::io;

fn main() {
    let mut fah = String::new();
    
    io::stdin()
        .read_line(&mut fah)
        .expect("read_line error");
    
    let fah : i32 = fah.trim().parse().expect("parse error");

    let cel = (fah - 32) * 5 / 9;

    println!("{} fahrenheit is {} celsius!", fah, cel);
}
