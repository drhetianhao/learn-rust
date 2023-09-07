#![allow(unused_variables)]

fn main() {
    let squared = i32::pow(8,2);
    println!("{}", squared);
    let float_integer = f32::powi(6.5, 3);
    let float_float = f32::powf(6.5, 3.14);
    println!("integer: {}", squared);
    println!("float to int: {}", float_integer);
    println!("float to float: {}", float_float);
}
