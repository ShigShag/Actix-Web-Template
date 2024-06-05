use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpResponse, Result};
use log::{error, info};
use std::sync::Arc;
use tera::{Context, Tera};

use crate::database::db::Database;
use crate::database::errors::DatabaseError;
use crate::get_user_id_from_session;
use crate::models::users::NewUser;
use crate::utils::render::{render_error, render_template};

use super::forms::RegisterForm;

pub async fn register(tera: web::Data<Tera>, session: Session) -> Result<HttpResponse> {
    // Check if user session already exists | If so redirect
    if let Some(_) = get_user_id_from_session!(session) {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/dashboard"))
            .finish());
    }

    let context = Context::new();

    render_template(&tera, "register/register.html", &context, StatusCode::OK)
}

pub async fn register_submit(
    db: web::Data<Arc<Database>>,
    session: Session,
    tera: web::Data<Tera>,
    post_data: web::Form<RegisterForm>,
) -> Result<HttpResponse, Error> {
    // Check if user session already exists | If so redirect
    if let Some(_) = get_user_id_from_session!(session) {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/dashboard"))
            .finish());
    }

    // Check if passwords are equal
    if post_data.password != post_data.password_confirm {
        return render_error(
            &tera,
            "Passwords do not match",
            "register/register.html",
            StatusCode::BAD_REQUEST,
        );
    }

    // Clone mail here since we move it into web::block and need it later
    let mail = post_data.email.clone();
    let password = post_data.password.clone();

    // Handle all database logic in one web::block
    let result = web::block(move || {
        // Check if a user already exists with the provided email
        match db.get_user_by_email(&mail) {
            // When an user already exists with that mail just return normal error
            Ok(_) => Err(DatabaseError::UserAlreadyExists(
                "An account already exists with that mail".to_string(),
            )),
            Err(DatabaseError::DieselError(diesel::result::Error::NotFound)) => {
                // User does not exist, proceed to create new user
                let new_user = NewUser::new(&mail, &password);
                match new_user {
                    Ok(user) => {
                        // Insert new user into the database
                        db.create_user(&user)
                    }
                    Err(err) => Err(err.into()),
                }
            }
            Err(err) => Err(err.into()),
        }
    })
    .await;

    match result {
        Ok(Ok(_)) => {
            info!("Created new user with email {}", &post_data.email);
            Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/login"))
                .finish())
        }

        // If user does not exist
        Ok(Err(DatabaseError::UserAlreadyExists(err))) => render_error(
            &tera,
            &err,
            "register/register.html",
            StatusCode::BAD_REQUEST,
        ),

        // If some error occurred within database operations
        Ok(Err(err)) => {
            error!("Database error: {}", err);
            render_error(
                &tera,
                "We are experiencing technical difficulties. Please try again later.",
                "register/register.html",
                StatusCode::BAD_REQUEST,
            )
        }
        Err(err) => {
            error!("Database error: {}", err);
            render_error(
                &tera,
                "We are experiencing technical difficulties. Please try again later.",
                "errors/error_page.html",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}
