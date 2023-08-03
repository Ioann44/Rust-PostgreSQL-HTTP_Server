use dotenvy::dotenv;
use postgres::{Client, NoTls};

use std::{env, sync::mpsc, thread};

pub fn establish_connection() -> Client {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Client::connect(&database_url, NoTls).expect("Connection to DB failed")
}

pub fn contains_valid_characters(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn get_json_from_table_async(table_name: &str) -> String {
    // Создаем канал для передачи данных между основным потоком и дочерним потоком
    let (sender, receiver) = mpsc::channel();

    // Создаем дочерний поток, в котором будем выполнять асинхронную работу
    let name_copy = table_name.to_string();
    thread::spawn(move || {
        let json_data = get_json_from_table(&name_copy);
        sender.send(json_data).unwrap();
    });

    let data = receiver.recv().unwrap();
    return data;
}

pub fn get_json_from_table(table_name: &str) -> String {
    let mut client = establish_connection();

    let query_result = client.query_one(
        &format!("select array_to_json(array_agg(row_to_json({table_name}))) from {table_name}"),
        &[],
    );

    let json_string;

    // В этом месте обрабатывается запрос к несуществующей таблице
    match query_result {
        Ok(row) => {
            let json_value: serde_json::Value = serde_json::from_value(row.get(0)).unwrap();
            json_string = serde_json::to_string(&json_value)
                .expect("Error serializing json vector to string");
        }
        Err(_error) => {
            json_string = "Where is no table with this name".to_string();
        }
    };

    return json_string;
}
