#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate chrono;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use self::models::NewTicket;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!(
        "Error connecting to {}",
        database_url
    ))
}

pub fn create_ticket(conn: &SqliteConnection, title: &str, description: &str) -> usize {
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
