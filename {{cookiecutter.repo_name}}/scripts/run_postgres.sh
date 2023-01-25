docker stop diesel_postgres
docker rm diesel_postgres
docker pull postgres:11.1
docker run --rm -P -p 127.0.0.1:5432:5432 -e POSTGRES_PASSWORD="1234" -d --name diesel_postgres postgres:alpine
echo DATABASE_URL=postgresql://postgres:1234@127.0.0.1:5432/postgres > .env
