#![allow(unused)]
use std::ops::{Add, Sub, Mul, Div};

fn main() {
    let sum = add(256, 262);
    println!("{}", sum);
}

fn add<T>(operand1: T, operand2: T) -> T
where T: Add<Output = T> + Sub<Output = T>
{
    operand1 + operand2
}