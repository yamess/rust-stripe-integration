use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::error::Error;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn run_migrations(database_url: &str) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let mut connection = PgConnection::establish(database_url)?;
    tracing::debug!("Running migrations");
    connection.run_pending_migrations(MIGRATIONS)?;
    tracing::debug!("Migrations complete");
    Ok(())
}
