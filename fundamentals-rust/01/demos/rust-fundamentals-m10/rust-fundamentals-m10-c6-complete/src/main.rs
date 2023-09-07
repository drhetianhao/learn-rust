use std::collections::HashSet;

fn main() {
    let mut flights:HashSet<&str> = HashSet::new();

    flights.insert("DA113\tto Boston departs at 06:20");
    flights.insert("DA98\tto London departs at 09:43");
    flights.insert("DA428\tto Salt Lake City departs at 12:05");
    flights.insert("DA41\tto Berlin departs at 15:30");
    flights.insert("DA2815\tto Nashville departs at 17:11");

    for flight in flights.iter() {
        println!("{:?}", flight);
    }
}
