#[macro_use]
extern crate log;

mod database;
mod client;
mod cli_args;

use thiserror::Error;

use client::HTTPNodeClient;
use grin_history_tools_models::schema;
use grin_history_tools_models::block::{BlockHeaderPrintable, model::BlockHeaderDb, model::ModelError};
use grin_core::global;
use database::{db_connection, PooledConnection};
use diesel::{Connection, RunQueryDsl};
use std::convert::TryFrom;


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


fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    // Sets options to environment variables
    let opt = {
        use structopt::StructOpt;
        cli_args::Opt::from_args()
    };

    let pool = database::pool::establish_connection(opt.clone());

	global::set_local_chain_type(global::ChainTypes::Mainnet);

    //let transport = Builder::new().url("http://localhost:13415/foreign");
    let node_url = "127.0.0.1:3413";
    let client = HTTPNodeClient::new(node_url, None);
    let res = client.get_header(1161601);
    println!("RES FROM NODE IS {:?}", res);

    let conn = db_connection(&pool).unwrap();
    let res = create_header(&conn, res.unwrap());
    println!("RES FROM DB IS {:?}", res);

}