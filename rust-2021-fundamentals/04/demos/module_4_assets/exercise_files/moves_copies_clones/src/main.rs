/**
 * Module 4 Exercise File - Moves, Copies, and Clones Demo
 */

#[derive(Debug, Clone, Copy)]
struct Coffee {
    id: i32,
    count: i32 
}


fn main() {
	// Moves/Copies primitives, structs
	let a = 1;
	let b = a;
	// The primitive value type is implicitly copied over here
	println!("a: {}", a);
	println!("b: {}", b);

	let string_a = String::from("Hello");
	let string_b = string_a;
	// Uncommenting this code will cause our program to not compile!
	// println!("string_a: {}", string_a);
	println!("string_b: {}", string_b);

	let string_c = String::from("World");
	let string_d = string_c.clone();
	println!("string_d: {}", string_d);

	{
		let greeting = String::from("Hello World!");
		println!("Greeting in scope: {}", greeting);
	}

	// Uncommenting this code will cause our program to not compile!
	// println!("Greeting: {}", greeting);


	let coffee_a = Coffee{id: 1, count: 40};
	let coffee_b = coffee_a;
	println!("coffee_b: {:?}", coffee_b);
	// Uncommenting this code will cause our program to not compile!
	// ... unless our struct is Copy
	println!("coffee_a: {:?}", coffee_a);

	let coffee_c = Coffee{id: 1, count: 40};
	let coffee_d = coffee_c.clone();
	println!("coffee_d: {:?}", coffee_d);

}
