#[macro_use]
extern crate log;

mod database;
mod client;
mod service;
mod cli_args;

use grin_core::global;

use crate::database::db_connection;
use crate::client::HTTPNodeClient;
use crate::service::block;


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
    let res = block::create_header(&conn, res.unwrap());
    println!("RES FROM DB IS {:?}", res);

}