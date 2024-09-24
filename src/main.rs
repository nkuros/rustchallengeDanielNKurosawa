#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;

use std::{env, io};

use actix_web::{middleware, App, Error, HttpServer};
use actix_web::web::Data;
use diesel::r2d2::ConnectionManager;
use diesel::{sql_query, PgConnection, RunQueryDsl};
use r2d2::{Pool, PooledConnection};
use errors::errors::ERROR_CONNECTION_POOL;

mod handlers;
mod models;
mod constants;
mod database;
mod errors;
mod controllers;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;


#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // let database_url: &str = env!("DATABASE_URL");
    let database_url = constants::constants::DATABASE_URL;

     // set up database connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");



   // make sure the database is set up

    if let Err(e) = set_database(&pool) {
        println!("Error: {}", e);
    }
        
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // Set up DB pool to be used with web::Data<Pool> extractor
            .app_data(Data::new(pool.clone()))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(handlers::node_handlers::populate)
            .service(handlers::node_handlers::get_nodes)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}


fn set_database(pool: &DBPool) -> Result<(), Error> {
        let conn = &mut pool.get().expect(ERROR_CONNECTION_POOL);
        let _ = sql_query(
        "
            CREATE TABLE IF NOT EXISTS nodes (
                public_key TEXT PRIMARY KEY,
                alias TEXT NOT NULL,
                channels BIGINT NOT NULL,
                capacity BIGINT NOT NULL,
                first_seen BIGINT NOT NULL,
                updated_at BIGINT NOT NULL,
                city hstore,
                country hstore
            )
        "
        ).execute(conn);

    Ok(())
}

