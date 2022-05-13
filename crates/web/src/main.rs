#[macro_use]
extern crate serde_derive;
//#[macro_use]
//extern crate log;
extern crate grin_history_tools_models;

mod cli_args;
mod database;
mod errors;
mod block;
mod seedcheck;
mod context;

use actix_web::{App, HttpServer, web::Data};
use actix_web::middleware::Logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Gets environment variables from `.env.example`
    dotenv::dotenv().ok();

    // Initiates error logger
    env_logger::init();

    // Sets options to environment variables
    let opt = {
        use structopt::StructOpt;
        cli_args::Opt::from_args()
    };

    // Database
    let pool = database::pool::establish_connection(opt.clone());

    // authorization
    /*let domain = opt.domain.clone();
    let cookie_secret_key = opt.auth_secret_key.clone();
    let secure_cookie = opt.secure_cookie;
    let auth_duration = time::Duration::hours(i64::from(opt.auth_duration_in_hour));*/

    // Server port
    let port = opt.port;

    // Server
    let server = HttpServer::new(move || {
        App::new()
            // Database
            .app_data(Data::new(pool.clone()))
            // Options
            .app_data(Data::new(opt.clone()))
            // Error logging
            .wrap(Logger::default())
            // Sets routes via secondary files
            .configure(block::route)
            .configure(seedcheck::route)
    })
    // Running at `format!("{}:{}",port,"0.0.0.0")`
    .bind(("0.0.0.0", port))
    .unwrap()
    // Starts server
    .run();

    eprintln!("Listening on 0.0.0.0:{}", port);

    // Awaiting server to exit
    server.await
}
