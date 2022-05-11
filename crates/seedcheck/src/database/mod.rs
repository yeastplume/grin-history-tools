use thiserror::Error;

pub mod pool;

use diesel::r2d2::PoolError;

type ConnectionManager = diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>;
pub type Pool = diesel::r2d2::Pool<ConnectionManager>;
pub type PooledConnection = diesel::r2d2::PooledConnection<ConnectionManager>;

pub fn db_connection(pool: &Pool) -> Result<PooledConnection, PoolConnError> {
    Ok(pool.get().map_err(|_| PoolConnError::UnableToConnectToDb)?)
}

#[derive(Error, Debug)]
pub enum PoolConnError {
    #[error("Unable to connect to database")]
    UnableToConnectToDb,
}

impl From<String> for PoolConnError {
    fn from(_: String) -> Self {
        PoolConnError::UnableToConnectToDb
    }
}

