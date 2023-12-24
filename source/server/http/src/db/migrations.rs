use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

// without the specified path it searches in the directory of the Cargo.toml
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migrations(database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = PgConnection::establish(database_url)?;
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run pending migrations.");

    Ok(())
}
