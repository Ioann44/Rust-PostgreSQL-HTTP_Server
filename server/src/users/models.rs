use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable)]
#[derive(Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub favorite_color: String
}
use crate::schema::users;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub favorite_color: String
}