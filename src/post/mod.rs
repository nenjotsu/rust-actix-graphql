mod handler;
pub mod model;
pub(crate) mod service;

use crate::post::handler::create;
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/post").service(web::resource("/").route(web::post().to(create))));
}
