use std::thread;

fn main() {
    let join_handle = thread::spawn(move || {
        outer_scope * 2
    });

    let result = join_handle.join();
    match result {
        Ok(value) => {
            println!("{}", value);
        }
        Err(_) => {}
    }
}