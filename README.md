# Actix-Web template

This template project uses the following technologies:

* Framework: Actix-Web
* Templating: Tera
* ORM: Diesel
* Database: Postgres SQL
* Cache: Garnet

## Usage

Create a `.env` file and set the following variables:

```bash
# Change these to appropiate values
POSTGRES_USER="superuser"
POSTGRES_PASSWORD="superpassword"
POSTGRES_DATABASE="main"

# Adjust based on the above
DATABASE_URL="postgresql://superuser:superpassword@localhost:5432/main"

# Garnet url still can use redis syntax
GARNET_URL="redis://127.0.0.1:6379"
```

---

### Running

Start database docker containers

```bash
sudo docker-compose up -d
```

Run Diesel migrations

```bash
diesel migration run
```

Run the application with cargo

```bash
cargo run --release
```

### Build docker container

```bash
# Network adresses must be changed for this to work
sudo docker build -t actix-web-template .
```