#![allow(unused_variables)]

fn main() {
    let duck = "Duck";
    let airlines = "Airlines";

    let airline_name = format!("{} {}", duck, airlines);
    println!("{}", airline_name);

    let mut slogan = String::new();
    slogan.push_str("We hit the ground");
    slogan.push(' ');
    slogan = slogan + "every time";
    println!("{}", slogan);
}
