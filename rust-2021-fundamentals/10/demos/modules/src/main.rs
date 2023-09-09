/**
 * Module 10 Exercise File - Modules Demo
 */

mod snack {

	#[derive(Debug)]
	pub struct Snack {
		pub name: String,
		pub rating: i32,
		pub cost: f64
	}
}

// Use private inline module
use snack::Snack;

use wbc::coffee::Coffee;
use wbc::coffee::espresso::Espresso; // Nested module
use wbc::tea::*;

fn main() {
	let test_snack = Snack {
		name: String::from("Popcorn"),
		rating: 99,
		cost: 2.49
	};

	println!("Snack: {:?}", test_snack);

	let test_coffee = Coffee {
		name: String::from("Drip coffee"),
		cost: 2.55,
		count: 10
	};

	println!("Coffee: {:?}", test_coffee);

	let test_espresso = Espresso {
		brand: String::from("Nespresso"),
		cost: 3.48
	};

	println!("Espresso: {:?}", test_espresso);

	let test_tea = Tea {
		brand: String::from("Harney & Sons"),
		cost: 1.48
	};

	println!("Tea: {:?}", test_tea);
}
