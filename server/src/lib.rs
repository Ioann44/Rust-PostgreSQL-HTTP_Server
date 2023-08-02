#[path = "users/models.rs"]
pub mod models;
#[path = "users/schema.rs"]
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NewUser, User};
use schema::users;

use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &mut PgConnection, name: String, favorite_color: String) -> User {
    let new_user = NewUser {
        name,
        favorite_color,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new user")
}

pub fn get_all_users_json(conn: &mut PgConnection) -> String {
    let results = users::table
        .limit(50)
        .select(User::as_select())
        .load(conn)
        .expect("Error loading posts");

    return serde_json::to_string_pretty(&results).expect("Serialization failed");
}

pub fn get_all_users_json_noconn() -> String {
    let connection = &mut establish_connection();
    return get_all_users_json(connection);
}
