#[macro_use]
extern crate log;

mod client;

use client::HTTPNodeClient;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    //let transport = Builder::new().url("http://localhost:13415/foreign");
    let node_url = "127.0.0.1:3413";
    let client = HTTPNodeClient::new(node_url, None);
    let res = client.get_header(1161601);
    println!("RES IS {:?}", res);
}