/**
 * Module 7 Exercise File - Handling Errors and Debugging Demo
 */

use std::{fs::{OpenOptions, File}, io::Result};


#[derive(Debug)]
struct Coffee {
    id: i32,
	count: i32
}

#[derive(Debug, PartialEq, Clone)]
struct MyCustomError {
	message: String
}

impl MyCustomError {
	fn new(message: &str) -> MyCustomError {
		MyCustomError { message: message.to_string() }
	}
}

fn open_file(path: &str) -> Result<File> {
	OpenOptions::new().read(true).open(path)
}

fn open_file_chain(file_one: &str, file_two: &str) -> Result<File> {
	open_file(file_one)?;
	open_file(file_two)
}

fn main() {
	let coffees = Vec::from([
		Coffee{id: 1000, count: 10},
		Coffee{id: 2000, count: 20},
		Coffee{id: 3000, count: 30}
	]);

	println!("Vector of Coffees: {:?}\n", coffees);

	// Access an invalid index from this vector - handle Option
	let maybe_coffee = coffees.get(4);

	let result = match maybe_coffee {
		Some(coffee) => Ok(coffee),
		None => Err(MyCustomError::new("Coffee did not exist!\n\n"))
	};

	match result {
		Ok(coffee) => println!("{:?}", coffee),
		Err(err) => print!("{}", err.message)
	};


	let open_file_result = open_file("file.txt");
	match open_file_result {
		Ok(file) => println!("{:?}", file),
		Err(err) => println!("sdfsfd") // Totally unexpected error for this program
		// Err(err) => panic!("{:?}", err) // Totally unexpected error for this program
	};

	let open_file_chain_result = open_file_chain("file.txt", "log.txt");
	match open_file_chain_result {
		Ok(last_file) => println!("{:?}", last_file),
		Err(err) => println!("{:?}\n", err)
	};
}



