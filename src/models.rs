extern crate diesel;

use super::schema::tickets;
use chrono::NaiveDateTime;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[derive(Queryable)]
pub struct Ticket {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created_at: NaiveDateTime,
}

impl Ticket {
    pub fn create(conn: &SqliteConnection, title: &str, description: &str) -> usize {
        use schema::tickets;

        let new_ticket = NewTicket {
            title: title,
            description: description,
        };

        diesel::insert(&new_ticket)
            .into(tickets::table)
            .execute(conn)
            .expect("Error saving new ticket")
    }
}


#[derive(Insertable)]
#[table_name = "tickets"]
pub struct NewTicket<'a> {
    pub title: &'a str,
    pub description: &'a str,
}
