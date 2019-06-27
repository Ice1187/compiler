use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
	println!("Guess the number!");

	let secret = rand::thread_rng().gen_range(1, 101);

	// println!("secret: {}", secret);

	loop {
		println!("Input your guess");

		let mut guess = String::new();

		io::stdin().read_line(&mut guess)
		    .expect("Failed to read the input.");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Invalid input, must be a number.");
				continue;
			},
		};

		println!("Your guess: {}", guess);

		match guess.cmp(&secret) {
			Ordering::Greater => println!("Too Big!"),
			Ordering::Less => println!("Too small!"),
			Ordering::Equal => {
				println!("You got it!");
				break;
			}
		}
	}
}