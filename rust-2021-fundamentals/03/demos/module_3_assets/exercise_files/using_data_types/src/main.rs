/**
 * Module 3 Exercise File - Using Data Types Demo
 */


// Custom Data Types
#[derive(Debug)]
struct Coffee {
    id: i32,
    name: String,
	temp: TempCategory
}

#[derive(Debug)]
enum TempCategory {
	HOT(Option<f64>),
	ICED(Option<f64>)
}


fn main() {

	// Integers
	let my_num: i32 = 32;
	println!("Integer: {}", my_num);
	
	// Converting strings to integers 
	let parsed_num: i32 = "123".parse().unwrap();
	println!("Parsed num: {}", parsed_num);
	println!("Integer to string: {}", parsed_num.to_string());
	
	// Floats
	let my_float: f32 = 10.5;
	println!("Floor: {}", my_float.floor());
	println!("Ceiling: {}", my_float.ceil());
	println!("Rounded: {}", my_float.round());

	// Working with floats and integers
	let my_int = my_float as i32 + 1;
	let my_new_float = 1 as f32 + my_float;
	println!("Coercing float to int: {}", my_int);
	println!("Coercing int to float: {}", my_new_float);

	// Characters
	let my_char = 'A';

	println!("Is uppercase: {}", my_char.is_uppercase());
	println!("Is lowercase: {}", my_char.is_lowercase());
	println!("Lowercase: {}", my_char.to_ascii_lowercase());
	println!("String version: {}", my_char.to_string());

	// Booleans
	let my_bool = true;
	assert_eq!(my_bool, true);

	// Tuples
	let my_tuple = ('A', 5, 10.5);
	println!("Char/Integer/Float: {}/{}/{}", my_tuple.0, my_tuple.1, my_tuple.2);

	// Destructing tuple values into variables
	let (letter, integer, float_num) = my_tuple;
	println!("Char/Integer/Float: {}/{}/{}", letter, integer, float_num);

	let nested_tuple = ((1, 2), (3, 4));
	let ((num1, num2), (num3, num4)) = nested_tuple;
	println!("(({}, {}), ({}, {}))", num1, num2, num3, num4);

	// Arrays
	let my_arr = [1, 2, 3, 4, 5];
	for num in my_arr {
		println!("Number: {}", num);
	}

	let same_value_arr: [i32; 1000] = [10; 1000];
	println!("Array: {:?}", same_value_arr);
	println!("First Element: {}", same_value_arr[0]);
	println!("Array Length: {}", same_value_arr.len());
	println!("Array size: {}", std::mem::size_of_val(&same_value_arr));

	// Structs
	let mut my_coffee = Coffee{
		id: 10,
		name: String::from("Riley"),
		temp: TempCategory::HOT(None)
	};

	my_coffee.id = 1000;

	let id = 10;
	let coffee_with_temp = Coffee{
		// Field init shorthand
		id,
		name: String::from("Riley"),
		temp: TempCategory::HOT(Some(103.2))
	};


	println!("My Coffee: {:?}", my_coffee);
	println!("Name/Id: {}/{}", my_coffee.name, my_coffee.id);
	println!("Temp: {:?}", my_coffee.temp);

	println!("Coffee With Temp: {:?}", coffee_with_temp.temp);

	// Combining structs
	let combined = Coffee {
		id: 2000,
		..coffee_with_temp // Base struct
	};

	println!("Combined: {:?}", combined);

}


