/**
 * Module 4 Exercise File - References and Slices Demo
 */

#[derive(Debug)]
struct Coffee {
    id: i32,
    count: i32 
}


fn main() {

	let mut a = 1;

	// Note - this is a value type stored on the stack - this won't alter the original
	fn increase(mut input: i32) {
		input += 20;
		println!("input parameter after increase: {}", input);
	}
	increase(a);
	println!("a after increase: {}", a);

	// Since this value type is passed via mutable reference - we can alter it
	fn increase_by_reference(input: &mut i32) {
		*input += 20;
	}
	increase_by_reference(&mut a);
	println!("a after increase: {}", a);


	fn alter_coffee(coffee: &mut Coffee, increase: i32) {
		coffee.count += increase;
	}

	fn print_coffee(coffee: &Coffee) {
		println!("My Coffee with ID {} and Count {}", coffee.id, coffee.count);
	}

	let mut my_coffee = Coffee{ id: 10, count: 50 };
	alter_coffee(&mut my_coffee, 500);
	print_coffee(&my_coffee);

	
	// Slices - array slices, string slices
	let str_slice = "Hello World";

	let my_str = String::from(str_slice);
	let slice_from_str = &my_str[0..5];
	println!("My string slice {}", slice_from_str);

	let my_arr = [1, 2, 3, 4, 5];
	let arr_slice = &my_arr[1..3];
	println!("My array slice {:?}", arr_slice);

	let mut my_mut_arr = [1, 2, 3, 4, 5];
	let mut mut_arr_slice = &mut my_mut_arr[0..3];
	mut_arr_slice[0] = 100000;
	println!("My mutable array slice {:?}", mut_arr_slice);
	println!("My mutable array {:?}", my_mut_arr);
}
