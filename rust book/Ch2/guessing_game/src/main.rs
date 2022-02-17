use std::io; // importing std::io library, ex) accept user input
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1..101);

	// println!("The secret number is: {}", secret_number);

	loop {
		println!("Please input your guess.");

		let mut guess = String::new(); // a place to store the user input

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");
		
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		
		println!("You guessed: {}", guess);

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}

	// let x = 5;
	// let y = 10;
	// println!("x = {} and y = {}", x, y);
}