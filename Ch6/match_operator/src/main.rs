fn main() {
    println!("value_in_cents: {}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?} {:?} {:?}", five, six, none);


    let some_u8_value = 5u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // _ placeholder is a pattern that will match any value (like default)
    }

}

fn value_in_cents(coin: Coin) -> i8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x { // has to have all possible cases or Rust will not compile
        None => None,
        Some(i) => Some(i + 1),
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    // Arkansas,
    // California,
    // Colorado,
    // Connecticut,
    // Delaware,
    // Florida,
    // Georgia,
    // Hawaii,
    // Idaho,
    // Illinois,
    // Indiana,
    // Iowa,
    // Kansas,
    // Kentucky,
    // Louisiana,
    // Maine,
    // Maryland,
    // Massachusetts,
    // Michigan,
    // Minnesota,
    // Mississippi,
    // Missouri,
    // Montana,
    // Nebraska,
    // Nevada,
    // New_Hampshire,
    // New_Jersey,
    // New_Mexico,
    // New_York,
    // North_Carolina,
    // North_Dakota,
    // Ohio,
    // Oklahoma,
    // Oregon,
    // Pennsylvania,
    // Rhode_Island,
    // South_Carolina,
    // South_Dakota,
    // Tennessee,
    // Texas,
    // Utah,
    // Vermont,
    // Virginia,
    // Washington,
    // West_Virginia,
    // Wisconsin,
    // Wyoming,
}