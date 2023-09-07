#![allow(unused_variables)]

fn main() {
    let bitwise_and = 86 & 27;
    println!("bitwise and: {}", bitwise_and);

    let bitwise_or = 86 | 27;
    println!("bitwise or:  {}", bitwise_or);

    let bitwise_xor = 86 ^ 27;
    println!("bitwise xor: {}", bitwise_xor);

    let left_shift = 86<<1;
    println!("left shift: {}", left_shift);

    let right_shift = 86>>1;
    println!("right shift: {}", right_shift);
}
