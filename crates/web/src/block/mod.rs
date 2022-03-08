use actix_web::web;

mod handler;

pub(super) fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/block").route(web::post().to(handler::get_block)));
        //.service(web::resource("/").route(web::get().to(handler::playground)));
}
