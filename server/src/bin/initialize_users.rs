use diesel::prelude::*;
use server::create_user;

fn main() {
    use server::schema::users::dsl::*;

    let connection = &mut server::establish_connection();

	// Удаление всех записей таблицы
    diesel::delete(users)
        .execute(connection)
        .expect("Error while erasing the table");

	// Добавление новых
    let data_sample = "Анна синий Максим зеленый Екатерина красный Иван желтый Ольга оранжевый Алексей фиолетовый София розовый Дмитрий серый Наталья коричневый Павел голубой".split_ascii_whitespace().collect::<Vec<&str>>();
	for i in (0..data_sample.len()).step_by(2) {
		create_user(connection,
			data_sample.get(i).expect("Name not readed").to_string(),
			data_sample.get(i+1).expect("Color not readed").to_string());
	}
}
