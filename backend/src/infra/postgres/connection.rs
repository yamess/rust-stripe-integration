use crate::prelude::*;

use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};
use std::sync::Arc;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection(database_url: &str) -> Arc<DbPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .min_idle(Some(2))
        .max_size(10)
        .connection_timeout(std::time::Duration::from_secs(30))
        .idle_timeout(Some(std::time::Duration::from_secs(300)))
        .max_lifetime(Some(std::time::Duration::from_secs(1800)))
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to create pool.");
    Arc::new(pool)
}

pub fn get_connection(pool: Arc<DbPool>) -> Result<DbConnection> {
    let connection = pool.get().map_err(|e| Error::Connection(e.to_string()))?;
    Ok(connection)
}
