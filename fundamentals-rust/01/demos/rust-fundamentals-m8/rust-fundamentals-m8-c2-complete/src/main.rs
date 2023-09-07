#![allow(unused)]

fn main() {
    let greater = return_greater(10, 5);
    println!("{}", greater);
}


fn return_greater(first: u8, second: u8) -> u8 {
    if first > second {
        first
    } else {
        second
    }
}