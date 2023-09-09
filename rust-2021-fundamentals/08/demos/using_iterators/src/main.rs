/**
 * Module 8 Exercise File - Using Iterators Demo
 */

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Coffee {
    id: i32,
	count: i32
}

fn main() {
	let coffees = Vec::from([
		Coffee{id: 1000, count: 10},
		Coffee{id: 2000, count: 20},
		Coffee{id: 3000, count: 30}
	]);

	let coffee_iter = coffees.iter();
	println!("Vector iterator: {:?}\n", coffee_iter);

	// Iterator adapters - some consume and some don't
	let total: i32 = coffee_iter.map(|coffee| coffee.count).sum();
	println!("Total count: {}\n", total);


	let coffee_map = HashMap::from([
		("Coffee1", Coffee{id: 1000, count: 10}),
		("Coffee2", Coffee{id: 2000, count: 40}),
		("Coffee3", Coffee{id: 3000, count: 500})
	]);
	let coffee_map_iter = coffee_map.iter();
	println!("Coffee map iterator: {:?}\n", coffee_map_iter);

	let filtered_coffees: Vec<Coffee> =
		coffee_map_iter
			.filter(|(_key, value)| value.count >= 40)
			.map(|(_key, value)| value)
			.cloned()
			.collect();

	for coffee in filtered_coffees {
		println!("Coffee: {:?}", coffee);
	}

	// This code won't compile because 'filter' takes ownership of this iterator
	for (key, value) in coffee_map_iter {
		println!("Original Coffee Entry: ({} | {:?})", key, value);
	}

	// This is just fine though!
	for (key, value) in coffee_map.iter() {
		println!("Original Coffee Entry: ({} | {:?})", key, value);
	}
}



