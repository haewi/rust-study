fn main() {
    let number = 7;

    if number < 5 {
        println!("true");
    }
    else {
        println!("false");
    }

    let condition = true;

    let number = if condition { 5 } else { 6 };
    println!("number is {}", number);

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for n in (1..4).rev() {
        println!("{}!", n);
    }
    println!("LIFTOFF!!");
}
