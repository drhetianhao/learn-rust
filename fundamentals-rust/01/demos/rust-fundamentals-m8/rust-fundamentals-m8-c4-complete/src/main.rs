#![allow(unused)]

fn main() {
    let airline_name = "Duck Airlines";

    let write_airline = |slogan: String| -> String {
        String::from(format!("{}. {}", airline_name, slogan))
    };

    let phrase =
        write_airline(String::from("Duck Airlines. We hit the ground every time."));
    println!("{}", phrase);
}