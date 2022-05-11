#[macro_use]
extern crate log;

mod database;
mod service;
mod cli_args;

use grin_core::global;

use crate::database::db_connection;
use crate::service::{seed, DbError};
use thiserror::Error;


fn main() -> Result<(), GrinSeedCheckError> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Sets options to environment variables
    let opt = {
        use structopt::StructOpt;
        cli_args::Opt::from_args()
    };

    let pool = database::pool::establish_connection(opt.clone());
	global::init_global_chain_type(global::ChainTypes::Mainnet);


    let conn = db_connection(&pool).unwrap();

    seed::update_seeds(&conn)?;

    Ok(())
}

#[derive(Error, Debug)]
pub enum GrinSeedCheckError {
	/// DbError
    #[error("{0}")]
	DbError(DbError),
}

impl From<DbError> for GrinSeedCheckError {
    fn from(e: DbError) -> Self {
        GrinSeedCheckError::DbError(e)
    }
}