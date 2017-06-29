extern crate ticket;
extern crate diesel;

use self::ticket::*;
use self::ticket::models::*;
use self::diesel::prelude::*;
use db;
use std::io::{stdin, Write, stdout};

pub fn index() {
    use self::schema::tickets::dsl::*;

    let connection = db::establish_connection();
    let results = tickets.load::<Ticket>(&connection).expect(
        "Error loading tickets",
    );

    // results
}

pub fn create() {
    let connection = db::establish_connection();

    print!("Enter a title: ");
    stdout().flush().unwrap();
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_right();

    print!("Enter an description: ");
    stdout().flush().unwrap();
    let mut description = String::new();
    stdin().read_line(&mut description).unwrap();
    let description = description.trim_right();

    println!("Creating ticket!");

    let _ = Ticket::create(&connection, title, description);
    println!("\nSaved {}", title);
}
