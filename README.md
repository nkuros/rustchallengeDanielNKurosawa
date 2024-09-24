cargo install diesel_cli --no-default-features --features postgres

 diesel migration generate --diff-schema create_nodes --database-url "postgres://postgres:admin@localhost:5432/postgres"

 ## Build tools & versions used

 Postgres 16

[package]
name = "nodes-challenge-bipa"
authors = ["Daniel N Kurosawa"]
edition = "2021"
version = "0.1.0"

[dependencies]
reqwest = { version = "0.12.7", features = ["json", "blocking", "rustls-tls"] }
diesel = { version = "2.2.4", features =["postgres", "r2d2", "chrono"]}
dotenvy = "0.15"
env_logger = "0.11.5"
actix-web = "4"
actix-rt = "2"
serde = { version = "1.0.106", features = ["derive"] }
serde_json = "1.0.51"
chrono = { version = "0.4", features = ["serde"] }
r2d2 = "0.8"
r2d2-diesel = "1.0"
http = "1.1.0"

## Steps to run the app
Initialize Database
docker-compose up -d db
docker compose build

Initialize application
cargo update
cargo build --release
cargo run

## What was the reason for your focus? What problems were you trying to solve?
Due to lack of familiarity with Rust, I had to guess what frameworks and packages to use and their integrations.

## How long did you spend on this project?
8 hours

## Did you make any trade-offs for this project? What would you have done differently with more time?
Could not store locations(city, country) > would store using postgres hstore type
Wasn't able to pass SSL certificates > Was only able to run db on a container and had to run application locally
Wasn't able to mock db access in order to write unit tests


## What do you think is the weakest part of your project?
Lack of unit testing
Application running locally
External Api call in code
## Is there any other information you’d like us to know?

