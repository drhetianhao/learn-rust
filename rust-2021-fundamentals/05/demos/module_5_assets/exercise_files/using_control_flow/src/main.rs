/**
 * Module 5 Exercise File - Control Flow Demo
 */

fn main() {
	// Expressions vs. Statements
	let a = 1;
	let b = a + 1;

	println!("Expression Evaluation: {}", a + 1 * b);

	// Control flow using conditional expressions
	if a == 1 {
		println!("Conditional Expression evaluated to true!");
	} else {
		println!("Conditional Expression evaluated to false!");
	}

	let result = if a + 1 == 2 { a } else { b };
	println!("Conditional Expression Evaluation: {}", result);


	let mut counter = 0;
	loop {
		if counter == 3 { break; };
		counter += 1;
	}

	loop {
		if counter < 10 {
			counter += 1000;
			println!("Continuing next loop iteration...");
			continue;
		}

		if counter > 1000 {
			println!("Counter is greater than 1000 - breaking.");
			break;
		}
	}

	// Nested loops with loop labels
	'one: loop {
		'two: loop {
			println!("Breaking out of Loop Two");
			break 'two;
		}
		println!("In Loop One");
		break 'one;
	}

	// Assigning values from a loop
	let mut start = 0;
	let result = loop {
		start += 1;
		if start == 20 {
			break start;
		}
	};
	println!("Loop Result: {}", result);

	let mut next_counter = 0;
	while next_counter < 10 {
		println!("Counter: {}", next_counter);
		next_counter += 1;
	}


	let my_arr = [1, 2, 3];
	let mut current_index = 0;
	while let Some(number) = my_arr.get(current_index) {
		println!("Index is valid - fetch number: {}", number);
		current_index += 1;
	}

	let my_arr = [1, 2, 3, 4, 5];

	for num in my_arr {
		println!("Number: {}", num);
	}

	let mut my_other_arr = [1, 2, 3];
	for num in &mut my_other_arr {
		*num += 1;
		println!("Mutated Number: {}", num);
	}
	println!("Mutated Array: {:?}", my_other_arr);

}
