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

pub struct Tickets;

impl Tickets {
    pub fn index(&self) -> Vec<Ticket> {
        use self::schema::tickets::dsl::*;

        let connection = establish_connection();
        let results = tickets.load::<Ticket>(&connection).expect(
            "Error loading tickets",
        );

        results
    }
}
