use rusqlite::{Connection, Result, Error};

struct Coffee {
    name: String,
    description: String,
    count: i32
}

fn main() -> Result<(), Error> {

    // Create our database if it doesn't exist
    let conn = Connection::open("coffee.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS coffee (
			id integer primary key,
			name text not null unique,
			description text,
			count integer
        )",
        [],
    )?;

	let coffee = Coffee {
		name: String::from("Drip"), 
		description: String::from("Nice, dark roast"),
		count: 32
	};

    conn.execute(
		"INSERT INTO coffee (name, description, count) values (?1, ?2, ?3)",
		[coffee.name, coffee.description, coffee.count.to_string()]
	)?;
    
    Ok(())
}
