extern crate ticket;
extern crate diesel;

use self::ticket::*;
use std::io::stdin;

fn main() {
    let connection = establish_connection();

    println!("Enter a name: ");

    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_right();

    println!("\nEnter an email: ");

    let mut email = String::new();
    stdin().read_line(&mut email).unwrap();
    let email = email.trim_right();

    println!("Creating user!");

    let _ = create_user(&connection, name, email);
    println!("\nSaved {}", name);
}
