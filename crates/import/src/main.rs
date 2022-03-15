#[macro_use]
extern crate log;

mod database;
mod client;
mod service;
mod cli_args;

use grin_core::global;

use crate::database::db_connection;
use crate::client::{HTTPNodeClient, HttpClientError};
use crate::service::{block, DbError};
use thiserror::Error;


fn main() -> Result<(), GrinImportError> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Sets options to environment variables
    let opt = {
        use structopt::StructOpt;
        cli_args::Opt::from_args()
    };

    let pool = database::pool::establish_connection(opt.clone());

	global::set_local_chain_type(global::ChainTypes::Mainnet);
    let node_url = "127.0.0.1:3413";
    let client = HTTPNodeClient::new(node_url, None);
    let conn = db_connection(&pool).unwrap();

    let tip = client.get_tip()?;
    info!("Tip is {}", tip.height);

    let to_height = tip.height;

    for n in 600000..to_height {
    //for n in 553150..553151 {
        let block_res = client.get_block(n);
        if let Ok(r) = block_res {
            block::add_block(&conn, r)?;
        }
        if n % 10000 == 0 {
            info!("Added {} blocks", n);
        }
    }

    /*let res = client.get_block(1161602);
    println!("RES FROM NODE IS {:?}", res);

    let res = block::add_block(&conn, res.unwrap());
    println!("RES FROM DB IS {:?}", res);*/
    Ok(())
}

#[derive(Error, Debug)]
pub enum GrinImportError {
	/// RPC Error
    #[error("{0}")]
	HttpClientError(HttpClientError),
	/// DbError
    #[error("{0}")]
	DbError(DbError),
}

impl From<HttpClientError> for GrinImportError {
    fn from(e: HttpClientError) -> Self {
        GrinImportError::HttpClientError(e)
    }
}

impl From<DbError> for GrinImportError {
    fn from(e: DbError) -> Self {
        GrinImportError::DbError(e)
    }
}