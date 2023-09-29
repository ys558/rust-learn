use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
	println!("Guess the number!");
	let secret_number = rand::thread_rng().gen_range(1..101);

	// print!("the secret number is {}\n", secret_number);

	loop {
		println!("Please input your number:");
	
		let mut guess = String::new();
		io::stdin()
			.read_line(&mut guess)
			.expect("fail to reade line");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err (_) => continue
		};

		println!("your guess: {}", guess);

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("{}", "too small".red()),
			Ordering::Equal => {
				println!("{}","you win".green());
				break;
			},
			Ordering::Greater => println!("{}", "too big".red()),
		}
	}
}
