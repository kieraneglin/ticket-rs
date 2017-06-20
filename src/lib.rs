#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use self::models::NewUser;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &SqliteConnection, name: &str, email: &str) -> usize {
    use schema::users;

    let new_user = NewUser {
        name: name,
        email: email,
    };

    diesel::insert(&new_user)
        .into(users::table)
        .execute(conn)
        .expect("Error saving new user")
}
