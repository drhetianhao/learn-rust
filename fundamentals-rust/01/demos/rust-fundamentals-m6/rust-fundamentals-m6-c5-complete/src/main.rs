
fn main() {
    let ndb_freq:u16 = 384;

    match ndb_freq {
        200..=500 => {
            println!("NDB frequency is valid");
        }
        _ => {
            println!("NDB frequency is not valid");
            match ndb_freq {
                ndb_freq if ndb_freq >= 200 && ndb_freq <= 500 => {
                    println!("NDB frequency is valid");
                }
                _ => {
                    println!("NDB frequency is not valid");
                }
            }
        }
    }
}
