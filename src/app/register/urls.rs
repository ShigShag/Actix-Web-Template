use actix_web::web;

use crate::app::register::views;

pub fn register_urls(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::get().to(views::register))
        .route("/register", web::post().to(views::register_submit));
}
