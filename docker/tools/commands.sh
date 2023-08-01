# создаёт и запускает контейнер с бд (-d отвечает за фоновый режим)
# единственная необходимая команда
docker-compose -f docker-compose.yml up -d
# удаление контейнера
docker-compose -f docker-compose.yml down