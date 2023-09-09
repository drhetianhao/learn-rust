use std::io::stdin;
use rust_syntax::add_constant;

fn main() {
	let mut number_str = String::new();
	println!("Enter your number:");
	
	stdin().read_line(&mut number_str).unwrap();
	
	// Parse our string after trimming off the excess characters
	let number: i32 = number_str.trim().parse().unwrap();
	println!("Calculation: {}", add_constant(number));

}
