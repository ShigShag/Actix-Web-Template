use actix_web::{error, http::StatusCode, web, Error, HttpResponse, Result};
use log::error;
use tera::{Context, Tera};

// Function to call when displaying error on the same page where it occurs, e.g. login or register
pub fn render_error(
    tera: &web::Data<Tera>,
    message: &str,
    template_path: &str,
    status_code: StatusCode,
) -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    context.insert("error_message", message);
    let rendered = tera
        .render(template_path, &context)
        .map_err(|_| error::ErrorInternalServerError("Failed to render template"))?;
    Ok(HttpResponse::build(status_code)
        .content_type("text/html")
        .body(rendered))
}

pub fn render_template(
    tera: &web::Data<Tera>,
    template_path: &str,
    context: &Context,
    status_code: StatusCode,
) -> Result<HttpResponse, Error> {
    match tera.render(template_path, context) {
        Ok(rendered) => Ok(HttpResponse::build(status_code)
            .content_type("text/html")
            .body(rendered)),
        Err(_) => {
            // Log the error when rendering fails
            error!("Failed to render template '{}'", template_path);

            // Call to render an error template
            render_error(
                &tera,
                "We are experiencing problems, please try again later.",
                "errors/error_page.html",
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}
