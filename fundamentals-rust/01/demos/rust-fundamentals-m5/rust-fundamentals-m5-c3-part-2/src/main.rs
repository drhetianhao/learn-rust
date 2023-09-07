#![allow(unused_variables)]

fn main() {
    let have_boarding_pass = true;
    let have_id = true;
    let can_board = have_boarding_pass && have_id;

    println!("Boarding Pass: {}, ID: {}", have_boarding_pass, have_id);
    println!("Can board plane: {}", can_board);
}
