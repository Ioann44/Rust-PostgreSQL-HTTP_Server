Сейчас не страшно передать .env как в docker/, так и в server/. Там можно изменить некоторые данные. При этом имя бд, пароль, и порт бд нужно менять в обоих, но так было проще

Из папки docker, он должен быть запущен. Создание контейнера БД
$ docker-compose -f docker-compose.yml up -d

Создаёт и наполняет данными таблицу users, запуск в папке server
$ cargo run --bin initialize_users

Старт сервера, запуск из WSL в папке server
$ cargo run --bin start_server

http://127.0.0.1:3000/table/users
http://127.0.0.1:3000/table/cats
http://127.0.0.1:3000/table/cars