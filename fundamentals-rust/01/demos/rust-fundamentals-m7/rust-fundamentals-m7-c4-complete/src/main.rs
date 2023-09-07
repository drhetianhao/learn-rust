
fn main() {
    let mut original = String::from("original value");
    println!("\nouter scope original: \t\"{}\"", original);

    {
        let next = &mut original;
        *next = String::from("next value");
        println!("inner scope next: \t\"{}\"", next);
        println!("inner scope original: \t\"{}\"", original);
    }

    println!("{}", next);
    println!("outer scope original: \t\"{}\"\n", original);
}
