use database;
use diesel;
use diesel::prelude::*;
use schema::tickets;
use models::{Ticket, NewTicket};

impl Ticket {
    pub fn create(title: &str, description: &str) -> Self {
        let connection = database::establish_connection();
        let new_ticket = NewTicket {
            title: title,
            description: description,
        };

        diesel::insert(&new_ticket)
            .into(tickets::table)
            .execute(&connection)
            .expect("Error saving new ticket");

        // Return the last ticket.  Not ideal, but ID isn't returned by `insert` for SQLite

        Ticket::last()
    }

    pub fn all() -> Box<Vec<Ticket>> {
        use schema::tickets::dsl::*;

        let connection = database::establish_connection();
        let results = tickets.load::<Ticket>(&connection).expect(
            "Error loading tickets",
        );

        // TODO: Revisit this.  Is this a good application for boxes?
        Box::new(results)
    }

    pub fn find(record: i32) -> Ticket {
        let connection = database::establish_connection();
        tickets::table
            .find(record)
            .first::<Ticket>(&connection)
            .expect("Error loading ticket")
    }

    pub fn delete(record: i32) -> usize {
        use schema::tickets::dsl::*;

        let connection = database::establish_connection();

        diesel::delete(tickets.filter(id.eq(record)))
            .execute(&connection)
            .expect("Error deleting posts")
    }

    pub fn last() -> Ticket {
        use self::tickets::dsl::{tickets, id};

        let connection = database::establish_connection();

        tickets.order(id.desc()).first(&connection).unwrap()
    }
}
