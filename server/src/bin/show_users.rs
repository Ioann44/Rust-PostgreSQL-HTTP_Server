use server::get_all_users_json;

fn main() {
    let connection = &mut server::establish_connection();
    let results = get_all_users_json(connection);
    println!("{}", results);
}
