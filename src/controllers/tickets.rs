extern crate ticket;
extern crate diesel;

use self::ticket::models::*;
use std::io::{stdin, Write, stdout};

pub fn index() {
    let tickets = Ticket::all();
    println!("{:#?}", tickets);
}

pub fn create() {
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

    let _ = Ticket::create(title, description);
    println!("\nSaved {}", title);
}
