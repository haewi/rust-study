fn main() {
    another_function(5, 6);

    let x = 5;
    
    let y = { // this block is an expression which evaluates to 4
        let x = 3;
        println!("x in block: {}", x);
        x + 1 // no semicolon at the end of expression
              // a semicolon will make it into a statement which does not return a value
    };

    println!("x out of block: {}", x);
    println!("y out of block: {}", y);

    println!();

    let x0 = five();
    println!("The value of x0 is: {}", x0);

    println!("The value of x is: {}", plus_one(5));
}

fn another_function(x: i32, y: i32){
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}