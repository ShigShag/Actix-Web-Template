pub mod login;
pub mod register;
pub mod dashboard;

pub fn register_urls(cfg: &mut actix_web::web::ServiceConfig) {
    login::urls::register_urls(cfg);
    register::urls::register_urls(cfg);
    dashboard::urls::register_urls(cfg);
}
