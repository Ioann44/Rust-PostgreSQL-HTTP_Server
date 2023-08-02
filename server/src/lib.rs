#[path = "users/models.rs"]
pub mod models;
#[path = "users/schema.rs"]
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::User;

use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::NewUser;

pub fn create_user(conn: &mut PgConnection, name: String, favorite_color: String) -> User {
    use crate::schema::users;

    let new_user = NewUser {
        name,
        favorite_color
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new user")
}
