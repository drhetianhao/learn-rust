
enum NavigationAids {
    NDB = 3,
    VOR = 2,
    VORDME = 5,
    FIX {name: String, latitude: f32, longitude: f32}
}

fn main() {
    println!("NDB:\t{}", NavigationAids::NDB as u8);
    println!("VOR:\t{}", NavigationAids::VOR as u8);
    println!("VORDME:\t{}", NavigationAids::VORDME as u8);
}
