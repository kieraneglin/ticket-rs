use super::schema::tickets;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

// #[derive(Queryable)]
// pub struct Ticket {
//     pub id: i32,
//     pub name: String,
//     pub email: String,
// }

#[derive(Insertable)]
#[table_name = "tickets"]
pub struct NewTicket<'a> {
    pub title: &'a str,
    pub description: &'a str,
}
