use super::schema::tickets;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Ticket {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created_at: NaiveDateTime,
}


#[derive(Insertable)]
#[table_name = "tickets"]
pub struct NewTicket<'a> {
    pub title: &'a str,
    pub description: &'a str,
}
