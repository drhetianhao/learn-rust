use std::{sync::{mpsc, Mutex, Arc}, thread::{self, JoinHandle}, time::Duration};

/**
 * Module 12 Exercise File - Concurrency Traits Demo
 */


// This struct implicitly implements "Send" because the i32 and String type is Send
// This struct also implicitly implements "Sync" for the same reason.
#[derive(Debug)]
struct Coffee {
    id: i32,
	count: i32,
	name: String
}

fn main() {
	// Message passing between threads using channels

	let (transmitter, receiver) =
		mpsc::channel();

	// Not capturing the thread handle just causes these threads to kick off - main thread
	// doesn't have to wait for them to finish
	thread::spawn(move || {
		for id in 0..20 {
			let coffee = Coffee { id: id + 1, count: 50, name: String::from("Drip") };
			// We can send coffees here because each field within Coffee is Send
			// Send let's us transfer ownership between threads
			transmitter.send(coffee).unwrap();
			thread::sleep(Duration::from_millis(500));
		}
	});

	let receiver_thread = thread::spawn(move || {
		for _ in 0..20 {
			let coffee = receiver.recv().unwrap();
			println!("Received coffee: {:?}", coffee);
			thread::sleep(Duration::from_millis(750));
		}
	});

	receiver_thread.join().unwrap();

	// Coffee, like Arc implements Sync and can safely be given access from multiple threads 
	let coffee =
		Arc::new(Mutex::from(Coffee {id: 0, count: 20, name: String::from("Drip")}));

	let mut thread_handles: Vec<JoinHandle<()>> = vec![];

	for _ in 0..5 {
		let coffee = Arc::clone(&coffee);

		let current_handle = thread::spawn(move || {
			let mut coffee = coffee.lock().unwrap();

			coffee.count += 5;
			println!("Coffee count after thread altered it: {}", coffee.count);
		});

		thread_handles.push(current_handle);
	}

	// Wait for all threads to finish mutating our shared-state
	for handle in thread_handles {
		handle.join().unwrap();
	}

	println!("Final count: {}", coffee.lock().unwrap().count);

	// Final Note:
	// You *can* implement Sync and Send traits manually but this is UNSAFE
	// You don't have to derive these directly since these traits are automatically
	// implemented when all sub-types implement them. 
}
