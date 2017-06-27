extern crate ticket;
extern crate diesel;

use ticket::*;
use self::models::*;
use diesel::prelude::*;

fn main() {
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
        println!("----------");
    }
}
