use actix_web::web;

mod handler;
pub(crate) mod service;

pub(super) fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/seedcheck")
            .service(web::resource("/latest").route(web::get().to(handler::latest_seedcheck))),
    );
}
