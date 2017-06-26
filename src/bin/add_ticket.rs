extern crate ticket;
extern crate diesel;

use self::ticket::*;
use std::io::stdin;

fn main() {
    let connection = establish_connection();

    println!("Enter a title: ");

    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_right();

    println!("\nEnter an description: ");

    let mut description = String::new();
    stdin().read_line(&mut description).unwrap();
    let description = description.trim_right();

    println!("Creating ticket!");

    let _ = create_ticket(&connection, title, description);
    println!("\nSaved {}", title);
}
