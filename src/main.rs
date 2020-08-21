extern crate rand;

use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
	println!("Guess the number!");
	
	let secret_number = rand::thread_rng()
		.gen_range(1, 101);
	let mut attempts = 0;
	
	loop {
		attempts += 1;
		
		println!("Please input your guess.");
		
		let mut guess = String::new();
		
		io::stdin().read_line(&mut guess)
		           .expect("failed to read line");
		
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("not a number!");
				continue;
			}
		};
		
		println!("You guessed: {}", guess);
		
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win! Attempts: {}", attempts);
				break;
			}
		}
	}
}
