use std::thread::{self, JoinHandle, current};
use std::sync::{mpsc, Mutex, Arc};
use std::time::Duration;

/**
 * Module 12 Exercise File - Concurrency Demo
 */


#[derive(Debug)]
struct Coffee {
    id: i32,
	count: i32
}

fn main() {

	// Spawning threads
	let thread_one = thread::spawn(|| {
		println!("Logging from Thread 1!");
	});


	// We can use the thread "builder" to build a new thread with a name
	let thread_two =
		thread::Builder::new().name("Thread 2".to_string()).spawn(|| {
			println!("Logging from Thread 2!");
		}).unwrap();

	// This thread will not complete until Thread 2 completes
	let thread_three = thread::spawn(|| {
		// Capture the underlying Thread object
		let two = thread_two.thread();
		println!("Thread 2 name and id: {:?}/{:?}", two.name(), two.id());

		thread_two.join().unwrap();
		println!("Logging from Thread 3!");
	});

	// Joining thread handles (waiting for threads to finish)
	// Any code after these 2 lines will not execute until are threads are complete
	thread_one.join().unwrap();
	thread_three.join().unwrap();

	// Message passing between threads using channels

	let (transmitter, receiver) =
		mpsc::channel();

	// Not capturing the thread handle just causes these threads to kick off - main thread
	// doesn't have to wait for them to finish
	thread::spawn(move || {
		for id in 0..20 {
			let coffee = Coffee { id: id + 1, count: 50 };
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


	// Shared-state between threads using mutexes
	// Here we use an Arc (short for Atomic reference counter) so that we can 
	// safely have shared ownership of the mutex between threads
	let message = Arc::new(Mutex::from(String::new()));

	// This vector helps us keep track of threads that we spawn within the main thread so that
	// we can wait for them all to finish
	let mut thread_handles: Vec<JoinHandle<()>> = vec![];

	// Spawn 5 threads - have each thread concurrently mutate our shared String
	for num in 0..5 {
		// Clone our atomic primitive - Arcs can safely be shared between threads
		let message = Arc::clone(&message);

		let current_handle = thread::spawn(move || {
			// Within the closure that each thread runs, lock our message so that other
			// threads can't mutate it while this thread is...
			let mut current_message = message.lock().unwrap();

			// Mutate the string
			let mut base_str = String::from(" Thread: ");
			base_str.push_str((num + 1).to_string().as_str());

			current_message.push_str(&base_str);
			println!("Message after thread altered it: {}", current_message);
		});

		thread_handles.push(current_handle);
	}

	// Wait for all threads to finish mutating our shared-state
	for handle in thread_handles {
		handle.join().unwrap();
	}

	println!("Final message: {}", message.lock().unwrap());

}
