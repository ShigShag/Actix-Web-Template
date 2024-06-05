use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpResponse, Result};
use log::{error, warn};
use std::sync::Arc;
use tera::{Context, Tera};

use crate::app::login::forms::LoginForm;
use crate::database::db::Database;
use crate::database::errors::DatabaseError;
use crate::get_user_id_from_session;
use crate::utils::argon2::verify_password;
use crate::utils::render::{render_error, render_template};

pub async fn login(tera: web::Data<Tera>, session: Session) -> Result<HttpResponse> {
    let context = Context::new();

    // Check if user session already exists | If so redirect
    if let Some(_) = get_user_id_from_session!(session) {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/dashboard"))
            .finish());
    }

    render_template(&tera, "login/login.html", &context, StatusCode::OK)
}

pub async fn login_submit(
    db: web::Data<Arc<Database>>,
    session: Session,
    tera: web::Data<Tera>,
    post_data: web::Form<LoginForm>,
) -> Result<HttpResponse, Error> {
    // Check if user session already exists | If so redirect
    if let Some(_) = get_user_id_from_session!(session) {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/dashboard"))
            .finish());
    }

    // Copy mail here since we do not want to move post_data
    let mail = post_data.email.clone();

    // Attempt to get user by email from the database
    let user_result = web::block(move || db.get_user_by_email(&mail)).await;

    match user_result {
        Ok(Ok(user)) => {
            // Check if given password is correct
            match verify_password(&post_data.password, &user.hashed_password) {
                true => {
                    // Create user session - Check for error when inserting data
                    if let Err(e) = session.insert("user_id", user.id) {
                        error!("Session error: {}", e);
                        return render_error(
                            &tera,
                            "We are experiencing problems, please try again later.",
                            "error/error_page.html",
                            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                        );
                    }

                    return Ok(HttpResponse::SeeOther()
                        .insert_header((LOCATION, "/dashboard"))
                        .finish());
                }
                false => {
                    warn!(
                        "Wrong password: {} for {}",
                        post_data.password, post_data.email
                    );

                    render_error(
                        &tera,
                        "Invalid mail or password",
                        "login/login.html",
                        StatusCode::BAD_REQUEST,
                    )
                }
            }
        }
        Ok(Err(DatabaseError::DieselError(err))) => match err {
            // Here we handle when everything is ok but entry cannot be found
            diesel::result::Error::NotFound => {
                error!("{} - {}", post_data.email, err);

                render_error(
                    &tera,
                    "Invalid mail or password",
                    "login/login.html",
                    StatusCode::BAD_REQUEST,
                )
            }
            // This is for other database errors which should not occur
            _ => {
                error!("{}", err);

                render_error(
                    &tera,
                    "We are experiencing problems, please try again later.",
                    "error/error_page.html",
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            }
        },
        Ok(Err(err)) => {
            // Connection error handling with appropriate HTTP status code
            error!("{}", err);

            render_error(
                &tera,
                "We are experiencing problems, please try again later.",
                "error/error_page.html",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
        // Blocking error
        Err(blocking_error) => {
            error!("Blocking error occurred: {:?}", blocking_error);
            render_error(
                &tera,
                "An internal server error occurred. Please try again later.",
                "error_template.html",
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

pub async fn logout(session: Session) -> Result<HttpResponse> {
    // Get the users session
    if let Some(_) = get_user_id_from_session!(session) {
        // Purge session
        session.purge();
    }
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .finish())
}
