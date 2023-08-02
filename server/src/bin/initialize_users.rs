use diesel::{prelude::*, sql_query};
use server::create_user;

fn main() {
    use server::schema::users::dsl::*;

    let connection = &mut server::establish_connection();

	// Попытка создания таблицы (diesel не позволяет делать это без миграций)
    let create_query = r#"
		CREATE TABLE IF NOT EXISTS users (
			id SERIAL PRIMARY KEY,
			"name" VARCHAR NOT NULL,
			favorite_color VarChar NOT NULL
	  	)"#;
    let _query_result = sql_query(create_query)
        .execute(connection)
        .expect("Error creating table");

    // Удаление всех записей таблицы
    diesel::delete(users)
        .execute(connection)
        .expect("Error while erasing the table");

    // Добавление новых
    let data_sample = "Анна синий Максим зеленый Екатерина красный Иван желтый Ольга оранжевый Алексей фиолетовый София розовый Дмитрий серый Наталья коричневый Павел голубой".split_ascii_whitespace().collect::<Vec<&str>>();
    for i in (0..data_sample.len()).step_by(2) {
        create_user(
            connection,
            data_sample.get(i).expect("Name not readed").to_string(),
            data_sample
                .get(i + 1)
                .expect("Color not readed")
                .to_string(),
        );
    }
}
