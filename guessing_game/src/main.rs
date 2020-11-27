use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

	let number = rand::thread_rng().gen_range(1, 100);
	println!("Guess the number");
	println!("Input your guess, pls.");
	
	loop {
		let mut guess = String::new();
		
		io::stdin()
			.read_line(&mut guess)
			.expect("Input error.");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(err) => {
				println!("{}", err);
				continue;
			},
		};

		match guess.cmp(&number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}
}
