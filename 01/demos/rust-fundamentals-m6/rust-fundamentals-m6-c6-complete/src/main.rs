
enum NavigationAids {
    NDB(u16),
    VOR(String, f32),
    VORDME(String, f32),
    FIX{name: String, latitude: f32, longitude: F32}
}

fn print_nav_aid(navaid: &NavigationAids) {
    match navaid {
        NavigationAids::NDB(khz) => {
            println!("NDB frequency is {} kilohertz", khz);
        }
        NavigationAids::VOR(id, freq) => {
            println!("VOR identifier is {} and it's frequency is {} kilohertz",
            id, freq);
        }
        NavigationAids::VORDME(id, freq) => {
            println!("VORDME identifier is {} and it's frequency is {} kilohertz",
            id, freq);
        }
        NavigationAids::FIX { name, latitude, longitude } => {
            println!("FIX {} is at {} latitude and {} longitude",
            name, latitude, longitude);
        }
    }
}

fn main() {
    let ndb_uwl = NavigationAids::NDB(385);
    let vor_dqn = NavigationAids::VOR(String::from("DQN"), 114.5);
    let vor_dme_sgh = NavigationAids::VORDME(String::from("SGH"), 113.2);
    let fix_tarry = NavigationAids::FIX {
        name: String::from("TARRY"),
        latitude: 40.05333,
        longitude: -83.91367
    };

    print_nav_aid(&ndb_uwl);
    print_nav_aid(&vor_dqn);
    print_nav_aid(&vor_dme_sgh);
    print_nav_aid(&fix_tarry);
}
