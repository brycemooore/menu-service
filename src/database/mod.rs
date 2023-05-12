use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::Connection;
use log::info;
use r2d2::Pool;
use diesel_migrations::{MigrationHarness, EmbeddedMigrations};




// The Postgres-specific connection pool managing all database connections.
pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;
const MIGRATION: EmbeddedMigrations = embed_migrations!();

pub fn get_pool(url: &str) -> PostgresPool {
    info!("Attempting to connect to database at {}", url);
    let migr = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(migr)
        .expect("could not build connection pool")
}

pub fn run_migrations(url: &str) {
    let mut connection = PgConnection::establish(url).expect("Error connecting to database");
    connection.run_pending_migrations(MIGRATION).unwrap();        
}
