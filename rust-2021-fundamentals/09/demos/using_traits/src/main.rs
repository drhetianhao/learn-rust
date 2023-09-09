/**
 * Module 9 Exercise File - Using Traits Demo
 */

#[derive(Debug, Clone, Copy)]
 enum TempCategory {
	HOT,
	ICED
 }

#[derive(Debug, Clone, Copy)]

 enum Roast {
	DARK,
	MEDIUM,
	LIGHT
 }


// Note: I can't derive from the built-in Copy trait here since String isn't Copy
#[derive(Debug, Clone)]
struct Coffee {
	name: String,
	temp: TempCategory,
	roast: Roast
}

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

struct Espresso {
	temp: TempCategory,
	brand: String
}

impl Brew for Espresso {
	fn brew(&self) -> () {
		println!("Brewing {}, {:?} espresso!", self.brand, self.temp);
	}
}

struct Tea {
	temp: TempCategory,
	origin: String,
	brand: String,
	rating: i32
}

impl Brew for Tea {
	fn brew(&self) -> () {
		println!(
			"Brewing {}, {:?} tea from {} with rating {}",
			self.brand,
			self.temp,
			self.origin,
			self.rating
		);
	}
}

struct Beer {}
impl Brew for Beer {}



trait Brew {
	fn brew(&self) -> () {
		println!("Brewing..."); // Default implementation
	}
}

fn main() {
	let coffee = Coffee{
		name: String::from("Verano"),
		temp: TempCategory::HOT,
		roast: Roast::DARK
	};
	coffee.brew();

	let espresso = Espresso{
		temp: TempCategory::HOT,
		brand: String::from("Nespresso") 
	};
	espresso.brew();

	let tea = Tea{
		origin: String::from("United Kingdom"),
		temp: TempCategory::HOT,
		brand: String::from("Harney & Sons"),
		rating: 99
	};

	tea.brew();

	let beer = Beer{};
	beer.brew();

	// This function takes any reference to an item that implements the Brew trait
	// This is an example of polymorphism in Rust - an object-oriented programming feature
	fn brew_drink(drink: &impl Brew) {
		drink.brew();
	}

	brew_drink(&coffee);
	brew_drink(&espresso);
	brew_drink(&tea);
	brew_drink(&beer);
}
