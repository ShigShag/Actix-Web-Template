use actix_files;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use std::sync::Arc;
use tera::Tera;

mod app;
mod database;
mod models;
mod schema;
mod utils;

use crate::database::db::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load env
    dotenv::dotenv().ok();

    // Create logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Get host and port
    let host = match std::env::var("HOST") {
        Ok(path) => path,
        Err(_) => String::from("0.0.0.0"), // Default adress
    };

    let port = match std::env::var("PORT") {
        Ok(path) => path,
        Err(_) => String::from("8000"), // Default port
    };

    let static_file_path = match std::env::var("STATIC_PATH") {
        Ok(path) => path,
        Err(_) => String::from("./static"),
    };

    let garnet_path = match std::env::var("GARNET_URL") {
        Ok(path) => path,
        Err(_) => String::from("redis://127.0.0.1:6379"),
    };

    let css_path = format!("{}/css", static_file_path);
    let js_path = format!("{}/js", static_file_path);

    let template_path = format!("{}/templates/**/*", static_file_path);

    let bind_address = format!("{}:{}", host, port);

    // TODO: Undo this to use permanent solution
    // let secret_key = Key::generate();
    let secret_key = Key::derive_from(&[0; 32]);

    let store = RedisSessionStore::new(garnet_path).await.unwrap();

    // Create new database pool | expect is ok since server cant run without db
    let database = Arc::new(Database::new().unwrap());

    // Create and start web server
    HttpServer::new(move || {
        let tera = Tera::new(&template_path.to_owned()).expect("Failed to initialize Tera");

        App::new()
            // Include css and javascript for dashboard
            .service(actix_files::Files::new("/css", css_path.clone()).show_files_listing())
            .service(actix_files::Files::new("/js", js_path.clone()).show_files_listing())
            // Include logger
            .wrap(Logger::default())
            // Session middleware
            .wrap(SessionMiddleware::new(store.clone(), secret_key.clone()))
            // Database clone
            .app_data(web::Data::new(database.clone()))
            // Templating
            .app_data(web::Data::new(tera))
            // Routing
            .configure(app::register_urls)
    })
    .bind(bind_address)?
    .workers(1)
    .run()
    .await
}
