mod config;
mod models;

use crate::models::Status;
use actix_web::{HttpServer, App, web, Responder};
use std::io;
use dotenv::dotenv;

// get status - /status
async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status {status: "OK".to_string()})
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Load the dotenv file
    dotenv().ok();

    // load config
    let config = crate::config::Config::from_env().unwrap();

    println!("Starting server at http://{}:{}", config.server.host, config.server.port);

    // Start the server
    HttpServer::new( || {
        App::new()
            .route("/status", web::get().to(status))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
