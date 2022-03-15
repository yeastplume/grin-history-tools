pub mod block;

use thiserror::Error;
use grin_history_tools_models::block::model::ModelError;

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

