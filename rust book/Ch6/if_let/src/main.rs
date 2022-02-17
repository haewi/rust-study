fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value { // same as previous code
        println!("three");
    }

    let coin = Coin::Quarter(UsState::Alabama);
    // let coin = Coin::Dime;


    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    println!("count: {}", count);

    // same as previous
    let coin = Coin::Quarter(UsState::Alabama);
    // let coin = Coin::Dime;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    }
    else {
        count += 1;
    }

    println!("count: {}", count);

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