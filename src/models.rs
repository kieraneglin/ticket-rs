use super::schema::{users, tickets};

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
}

#[derive(Insertable)]
#[table_name = "tickets"]
pub struct NewTicket<'a> {
    pub title: &'a str,
    pub description: &'a str,
}
