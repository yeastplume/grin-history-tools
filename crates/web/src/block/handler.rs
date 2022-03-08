use crate::cli_args::Opt;
use crate::database::{db_connection, Pool};
use actix_web::{error, web, Error, HttpResponse};

pub(super) async fn get_block(
    pool: web::Data<Pool>,
    _opt: web::Data<Opt>,
) -> Result<HttpResponse, Error> {
    let _db_pool = db_connection(&pool)?;

    /*let opt = opt.into_inner().as_ref().clone();
    let ctx = Context::new(db_pool, opt);*/

    let res = "{\"result\" = \"test\"}";
    let json = serde_json::to_string(&res).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}
