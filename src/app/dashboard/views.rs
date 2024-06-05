use crate::get_user_id_from_session;
use crate::utils::render::render_template;
use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Result};
use tera::{Context, Tera};

pub async fn dashboard(tera: web::Data<Tera>, session: Session) -> Result<HttpResponse> {
    // Check if user has no session | If that is the case send back to login
    if get_user_id_from_session!(session).is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .finish());
    }

    let context = Context::new();

    render_template(&tera, "dashboard/dashboard.html", &context, StatusCode::OK)
}
