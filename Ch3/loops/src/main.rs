fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
            // break "hi";
        }
    };

    println!("The result is {}", result);

    println!();

    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number-=1;
    }
    println!("LIFTOFF!!!");
}
