use crate::context::RequestContext;
use crate::errors::ServiceError;
use crate::seedcheck::service::get_seedcheck;

use actix_web::HttpResponse;

pub(super) async fn latest_seedcheck(
    context: RequestContext,
) -> Result<HttpResponse, ServiceError> {
    get_seedcheck::get_seedcheck(&context).map(|res| HttpResponse::Ok().json(&res))
}
