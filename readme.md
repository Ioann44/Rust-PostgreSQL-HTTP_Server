Сейчас не страшно передать .env как в docker/, так и в server/. Там можно изменить некоторые данные. При этом имя бд, пароль, и порт бд нужно менять в обоих, но так было проще

Из папки docker, он должен быть запущен. Создание контейнера БД
$ docker-compose -f docker-compose.yml up -d

Необоходимо для работы diesel, запускается из WSL
В теории можно установить пакет для postgresql на windows, но там у всех проблемы, что msvc не может найти libpq.lib и добавление в path не помогает
$ sudo apt install libpq-dev

Создаёт и наполняет данными таблицу users, запуск из WSL в папке server
$ cargo run --bin initialize_users

Можно проверить данные таблицы, запуск из WSL в папке server
$ cargo run --bin show_users

Старт сервера, запуск из WSL в папке server
$ cargo run --bin start_server

http://localhost:3000/table/users