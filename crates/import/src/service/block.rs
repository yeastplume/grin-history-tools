use thiserror::Error;

use grin_history_tools_models::schema;
use grin_history_tools_models::block::{BlockHeaderPrintable, model::BlockHeaderDb, model::ModelError};
use diesel::{Connection, RunQueryDsl};
use std::convert::TryFrom;

use crate::database::PooledConnection;

pub fn create_header(
	db: &PooledConnection,
	hp: BlockHeaderPrintable,
) -> Result<BlockHeaderDb, DbError> {
	use schema::headers::dsl::headers;

	db.transaction::<BlockHeaderDb, DbError, _>(|| {
		let inserted_header: BlockHeaderDb = diesel::insert_into(headers)
			.values(BlockHeaderDb::try_from(hp)?)
			.get_result(db)?;

		Ok(inserted_header)
	})
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Diesel Error {0}")]
    DieselError(String),
    #[error("Model Error {0}")]
    ModelError(String),
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

