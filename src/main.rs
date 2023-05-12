use actix::{Addr, SyncArbiter};
use actix_web::{middleware::Logger, App,  HttpServer, web};
use actors::DbActor;
use dotenv::dotenv;
use humantime;
use log::info;
use std::env;
use std::time::SystemTime;
use crate::database::PostgresPool;

#[macro_use]
extern crate diesel_migrations;

mod database;
mod actors;
mod models;
mod schema;

pub struct AppState {
    pub db: Addr<DbActor>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env::set_var("RUST_LOG", "actix_web=debug");

    match setup_logger() {
        Err(e) => panic!("{}", format!("Error setting up logger {}", e)),
        _ => (),
    };

    let url = env::var("DATABASE_URL").expect("no DB URL");
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    database::run_migrations(&url);
    let pool: PostgresPool = database::get_pool(&url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    info!("Server starting at {}:{}", host, port);

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState{db: db_addr.clone()}))
            .wrap(Logger::default())
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}