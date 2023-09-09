/**
 * Module 6 Exercise File - Using Functions Demo
 */


#[derive(Debug)]
struct Coffee {
    id: i32,
	count: i32
}

impl Coffee {
	fn increase_count(&mut self, amount: i32) {
		self.count += amount;
		self.print();
	}

	fn print(&self) {
		println!("Increasing count... Coffee {} has count {}", self.id, self.count);
	}

	// Associated function
	fn new(id: i32, count: i32) -> Coffee {
		Coffee {
			id,
			count
		}
	}
}

fn main() {

	// Named functions
	fn add(a: f32, b: f32) -> f32 {
		a + b
	}

	println!("Addition result: {}", add(3.5, 8.6));


	// Closures and Higher-order functions

	let add_ten = |x: i32| {
		x + 10
	};

	let even_numbers: Vec<i32> =
		(1..10).map(add_ten)
			   .filter(|x| x % 2 == 0)
			   .collect();

	println!("Even numbers: {:?}", even_numbers);

	let outer_var = 1;
	let my_closure = |x: i32| x * 10 + outer_var;

	// If you uncomment this code and make the outer_var variable mutable
	// it will not compile - value is now owned by the closure
	// outer_var += 1;

	println!("Closure with captured env result: {}", my_closure(5));
	println!("Closure with captured env result: {}", my_closure(10));

	// Methods and associated functions
	let mut my_coffee = Coffee::new(1, 1000);

	my_coffee.increase_count(1000);
}
