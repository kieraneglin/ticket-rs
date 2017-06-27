use super::schema::tickets;
use std::time::SystemTime;
// #[derive(Queryable)]
// pub struct User {
//     pub id: i32,
//     pub name: String,
//     pub email: String,
// }


#[derive(Queryable)]
pub struct Ticket {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created_at: SystemTime,
}

#[derive(Insertable)]
#[table_name = "tickets"]
pub struct NewTicket<'a> {
    pub title: &'a str,
    pub description: &'a str,
}
