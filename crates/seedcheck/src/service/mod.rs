pub mod seed;

use thiserror::Error;
use grin_history_tools_models::block::model::ModelError;
use grin_p2p as p2p;
use grin_store as store;

#[derive(Error, Debug)]
pub enum SeedCheckError {
    #[error("Seed Connect Error {0}")]
    SeedConnectError(String),
    #[error("Grin Store Error {0}")]
    StoreError(String),
}

impl From<p2p::Error> for SeedCheckError {
    fn from(e: p2p::Error) -> Self {
        SeedCheckError::SeedConnectError(format!("{:?}", e))
    }
}

impl From<store::lmdb::Error> for SeedCheckError {
    fn from(e: store::lmdb::Error) -> Self {
        SeedCheckError::StoreError(format!("{:?}", e))
    }
}



#[derive(Error, Debug)]
pub enum DbError {
    #[error("Diesel Error {0}")]
    DieselError(String),
    #[error("Model Error {0}")]
    ModelError(String),
    #[error("Data Error {0}")]
    DataError(String),
}

impl From<diesel::result::Error> for DbError {
    fn from(e: diesel::result::Error) -> Self {
        DbError::DieselError(format!("{}", e))
    }
}

impl From<ModelError> for DbError {
    fn from(e: ModelError) -> Self {
        DbError::ModelError(format!("{}", e))
    }
}

impl From<String> for DbError {
    fn from(e: String) -> Self {
        DbError::DataError(e.into())
    }
}

