// pub mod posts;
// extern crate ticket;
// use self::ticket::*;

// pub fn main() {
//     println!("Hello from con#posts {:?}");
// }

extern crate ticket;
extern crate diesel;


use self::ticket::*;
use self::ticket::models::*;
use self::diesel::prelude::*;
use std::io::{stdin, Write, stdout};

pub fn index() {
    use self::schema::tickets::dsl::*;

    let connection = establish_connection();
    let results = tickets.load::<Ticket>(&connection).expect(
        "Error loading tickets",
    );

    println!("Displaying {} tickets", results.len());
    for ticket in results {
        println!("----------");
        println!("{}", ticket.title);
        println!("{}", ticket.description);
        println!("{}", ticket.created_at);
        println!("----------");
    }
}

pub fn create() {
    let connection = establish_connection();

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

    let _ = create_ticket(&connection, title, description);
    println!("\nSaved {}", title);
}
