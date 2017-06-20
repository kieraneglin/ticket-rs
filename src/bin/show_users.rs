extern crate ticket;
extern crate diesel;

use ticket::*;
use self::models::*;
use diesel::prelude::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users.load::<User>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.name);
        println!("{}", user.email);
        println!("--------------------");
    }
}
