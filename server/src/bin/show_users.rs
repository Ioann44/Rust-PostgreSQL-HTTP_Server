use server::models::*;
use diesel::prelude::*;

fn main() {
    use server::schema::users::dsl::*;

    let connection = &mut server::establish_connection();
    let results = users
        .limit(50)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for user in results {
		let mut fav_col = user.favorite_color;
        println!("{}", user.name);
        println!("-----------\n");
        println!("{}", fav_col.get_or_insert("None".to_string()));
       }
}