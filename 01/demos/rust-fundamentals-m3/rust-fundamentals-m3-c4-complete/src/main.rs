#![allow(unused_variables)]

fn main() {
    let location = ("KCLE", 41.4094069, -81.8546911);
    let (name, latitude, longitude) = location;
    println!("Location name: {}, latitude: {}, longitude: {}",
             name, latitude, longitude);
}
