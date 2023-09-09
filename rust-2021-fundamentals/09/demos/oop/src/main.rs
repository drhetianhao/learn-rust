/**
 * Module 9 Exercise File - Object-oriented Programming Demo
 */

mod coffee {

	#[derive(Debug)]
	pub enum TempCategory {
		HOT,
		ICED
	}

	#[derive(Debug, Copy, Clone)]

	pub enum Roast {
		DARK,
		MEDIUM,
		LIGHT
	}


	#[derive(Debug)]
	pub struct Coffee {
		pub name: String,
		pub temp: TempCategory,
		pub roast: Roast,
		cost: f64,
		count: i32,
		total: f64
	}

	// Encapsulation of the calculation of total and getting/setting of other fields
	impl Coffee {

		// This is akin to a public constructor - notice how we calculate total behind the scenes
		pub fn new(name: String, temp: TempCategory, roast: Roast, cost: f64, count: i32) -> Coffee {
			Coffee {
				name,
				temp,
				roast,
				cost,
				count,
				total: cost * count as f64
			}
		}

		pub fn get_total(&self) -> f64 {
			self.total
		}

		pub fn set_cost(&mut self, cost: f64) {
			self.cost = cost;
			self.total = self.cost * self.count as f64;
		}

		pub fn set_count(&mut self, count: i32) {
			self.count = count;
			self.total = self.cost * self.count as f64;
		}
	}

	impl Drink for Coffee {
		fn drink(&self) -> () {
			println!(
				"Drinking {:?}, {:?} roast coffee named {}!",
				self.temp,
				self.roast,
				self.name
			);
		}

	}

	// Now we have to also implement Drink - this is the closest thing to inheritance provided
	impl Brew for Coffee {
		fn brew(&self) -> () {
			println!(
				"Brewing {:?}, {:?} roast coffee named {}!",
				self.temp,
				self.roast,
				self.name
			);
		}

	}

	// Pseudo-inheritance via supertraits
	pub trait Brew: Drink {
		fn brew(&self) -> () {
			println!("Brewing..."); // Default implementation
		}
	}

	pub trait Drink {
		fn drink(&self) -> () {
			println!("Drinking"); // Default implementation
		}
	}

}

use coffee::*;

fn main() {
	// Encapsulation test
	let mut coffee = Coffee::new(
		String::from("Verano"),
		TempCategory::HOT,
		Roast::DARK,
		3.50,
		50
	);

	// This code won't compile! The cost field is private!
	// coffee.cost = coffee.cost + 10;

	coffee.set_cost(4.50);
	println!("New total: {}", coffee.get_total());

	coffee.set_count(500);
	println!("New total: {}", coffee.get_total());

	// Pseudo-inheritance via traits test	
	fn brew_then_drink(drink: &impl Brew) {
		drink.brew();
		drink.drink();
	}

	brew_then_drink(&coffee);

}
