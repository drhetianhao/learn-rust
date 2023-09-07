#![allow(unused_variables)]

fn main() {
    let have_driver_license = false;
    let have_passport = true;
    let have_proof = have_driver_license || have_passport;

    let have_boarding_pass = false;
    let have_id = have_proof;
    let can_board = have_boarding_pass && have_id;

    println!("Boarding Pass: {}, ID: {}", have_boarding_pass, have_id);
    println!("Can board plane: {}", can_board);


}
