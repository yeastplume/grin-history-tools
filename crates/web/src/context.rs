use crate::cli_args::Opt;
use crate::database::{db_connection, Pool, PooledConnection};

use actix_web::dev::Payload;
use actix_web::web;
use actix_web::{Error, FromRequest, HttpRequest};

pub struct RequestContext {
	pub opt: web::Data<Opt>,
	pub db: PooledConnection,
}

impl FromRequest for RequestContext {
	type Error = Error;
	type Future = futures::future::Ready<Result<Self, Self::Error>>;

	fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
		let opt = web::Data::<Opt>::from_request(req, pl).into_inner();
    	let pool = web::Data::<Pool>::from_request(req, pl).into_inner();

		futures::future::ready(Ok(RequestContext {
			opt: opt.unwrap(),
			db: db_connection(&pool.unwrap()).unwrap(),
		}))
	}
}