#[path ="../db_data.rs"]
mod db_data;

fn main() {
    let mut client = server::establish_connection();

    for (table_name, query, data) in &[
        ("users", "\"name\" VARCHAR, favorite_color VARCHAR", db_data::USERS_DATA),
        ("cats", "\"name\" VARCHAR, age SMALLINT, breed VARCHAR, temper VARCHAR", db_data::CATS_DATA),
        ("cars", "brand VARCHAR, model VARCHAR, year SMALLINT", db_data::CARS_DATA)
    ] {
        // Удаление таблиц
        let _ = client.batch_execute(&format!("DROP TABLE IF EXISTS {table_name};"))
            .expect(&format!("Table {table_name} not deleted"));

        // Создание их заново
        let _ = client.batch_execute(&format!("
            CREATE TABLE {table_name} (
                id SERIAL PRIMARY KEY,
                {query}
            )"))
            .expect(&format!("Table {table_name} not created"));

        // Наполнение данными
        // Создание строки с типами
        let cols_and_types: Vec<&str> = query.split_ascii_whitespace().collect();
        let mut types_string = "".to_owned();
        types_string += cols_and_types.get(0).unwrap();
        for i in (2..cols_and_types.len()).step_by(2) {
            types_string += ", ";
            types_string += cols_and_types.get(i).unwrap();
        }
        // Цикл с созданием строки данных
        let data_sample = data.lines().collect::<Vec<&str>>();
        let step = data_sample.len() / 10;
        for i in (0..data_sample.len()).step_by(step) {
            let mut values_string = "".to_owned();
            for j in 0..step {
                let value_j = data_sample.get(i + j).unwrap();
                values_string += "\'";
                values_string += value_j;
                values_string += "\'";
                if j != step - 1 {
                    values_string += ", ";
                }
            }
            let _ = client.execute(&format!("INSERT INTO {table_name} ({types_string}) VALUES ({values_string})"), &[])
                .expect(&format!("Value not inserted into {table_name}"));
        }
    }
}
