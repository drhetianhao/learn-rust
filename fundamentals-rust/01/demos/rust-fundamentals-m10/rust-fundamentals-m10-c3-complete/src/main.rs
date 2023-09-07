
fn main() {
    let mut flights: Vec<&str> = Vec::new();

    flights.push("DA113\tto Boston departs at 06:20");
    flights.push("DA98\tto London departs at 09:43");
    flights.push("DA428\tto Salt Lake City departs at 12:05");
    flights.push("DA41\tto Berlin departs at 15:30");
    flights.push("DA2815\tto Nashville departs at 17:11");

    flights.insert(2, "DA918\tto Orlando departs at 11:12");

    for flight in flights.iter() {
        println!("{}", flight);
    }
}
