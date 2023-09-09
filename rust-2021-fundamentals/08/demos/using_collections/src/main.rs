/**
 * Module 8 Exercise File - Using Collections Demo
 */

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Coffee {
    id: i32,
	count: i32
}

fn main() {
	// Creating Vectors

	// We have to specify a type for an empty vector
	let mut empty_vector = Vec::new();

	empty_vector.push(1);


	let mut prime_nums = vec![2, 3, 5, 7];

	let mut even_nums = Vec::from([2, 4, 6]);

	let odd_nums: Vec<i32> = Vec::with_capacity(50);
	println!("Capacity of odd numbers: {:?}\n", odd_nums.capacity());

	prime_nums.push(11);
	println!("Prime numbers: {:?}\n", prime_nums);

	let maybe_removed = even_nums.pop();
	println!("Even numbers: {:?}\n", even_nums);

	let maybe_num = even_nums.get(0);
	match maybe_num {
		Some(num) => println!("Retrieved number {}", num),
		None => println!("No number at this index!")
	}

	match maybe_removed {
		Some(removed_num) => println!("Removed {}", removed_num),
		None => println!("No number removed")
	}


	// Creating HashMaps and basic operations
	let mut my_map = HashMap::new();
	my_map.insert(1 , "Hello");
	my_map.insert(2 , "World");

	let maybe_removed_val = my_map.remove(&8);
	match maybe_removed_val {
		Some(removed) => println!("Removed entry with value: {}", removed),
		None => println!("Key did not exist!")
	}

	match my_map.get(&1) {
		Some(str_slice_ref) => println!("Value: {}", *str_slice_ref),
		None => println!("Entry did not exist")
	}

	let my_coffee_map = HashMap::from([
		("Drip", 2.99),
		("Espresso", 4.50)
	]);
	println!("My coffee map: {:?}\n", my_coffee_map);

	let capacity_map: HashMap<i32, &str> = HashMap::with_capacity(10);
	println!("My coffee map with capacity: {:?}\n", capacity_map.capacity());

}
