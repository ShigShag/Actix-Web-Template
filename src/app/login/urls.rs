use actix_web::web;

use crate::app::login::views;

pub fn register_urls(cfg: &mut web::ServiceConfig) {
    cfg.route("/login", web::get().to(views::login))
        .route("/login", web::post().to(views::login_submit))
        .route("/logout", web::get().to(views::logout))
        .route("/", web::get().to(views::login));
}
