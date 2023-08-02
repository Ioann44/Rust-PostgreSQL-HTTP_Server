use diesel::prelude::*;
use server::models::*;

fn main() {
    use server::schema::users::dsl::*;

    let connection = &mut server::establish_connection();
    let results = users
        .limit(50)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading posts");

    let serialized_result = serde_json::to_string_pretty(&results).expect("Serialization failed");
    println!("{}", serialized_result);
}
