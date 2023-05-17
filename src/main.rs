
use actix_web::{middleware::Logger, App,  HttpServer, web};
use database::Database;
use dotenv::dotenv;
use humantime;
use log::{info, warn};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::time::SystemTime;


mod database;
mod models;
mod vm;
mod api;
mod service;
mod dto;
mod error;

pub struct AppState {
    pub db: Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env::set_var("RUST_LOG", "actix_web=debug");

    match setup_logger() {
        Err(e) => panic!("{}", format!("Error setting up logger {}", e)),
        _ => (),
    };

    //db setup
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be specified as an environment variable");
    let pool = get_database_connecntion(&url).await;
    run_database_migrations(&pool).await;

    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    info!("Server starting at {}:{}", host, port);

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState {db: Database::new(pool.clone())}))
        .service(api::get_menu_by_id)
        .service(api::post_menu)
        .service(api::post_item)
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

async fn get_database_connecntion(url: &str) -> Pool<Postgres> {
    match PgPoolOptions::new()
    .max_connections(10)
    .connect(&url)
    .await
    {
        Ok(pool) => {
            info!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            warn!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    }
}

async fn run_database_migrations(pool: &Pool<Postgres>) {
        // run migrations
        match sqlx::migrate!("./migrations").run(pool).await {
            Ok(_) => info!("✅ Database migration successful!"),
            Err(err) => {
                warn!("🔥 Database migration failed: {:?}", err);
                std::process::exit(1);
            }
        };
}