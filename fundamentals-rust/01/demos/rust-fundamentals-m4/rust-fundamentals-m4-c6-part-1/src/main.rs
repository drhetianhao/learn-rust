#![allow(unused_variables)]

fn main() {
    let scope_test = "outer scope";
    println!("{}", scope_test);
    {
        let scope_test = "inner scope";
        println!("{}", scope_test);
    }
    println!("{}", scope_test);
}
