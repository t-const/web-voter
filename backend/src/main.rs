use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use log::info;
use std::env;
use std::sync::Arc;
use tokio_postgres::NoTls;
use crate::handlers::{get_room, get_rooms, delete_room, new_room, get_user, get_users};

mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init();

    info!("Starting Actix Web server...");

    dotenv().ok();

    // Load database URL from environment variables
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    // Establish a connection to the database
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .expect("Failed to connect to database");
    
    let client = Arc::new(client);

    info!("Database connection established successfully");

    // Spawn the connection to run in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Start the Actix Web server
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .route("/user", web::get().to(get_user))
            .route("/users", web::get().to(get_users))
            .route("/room", web::get().to(get_room))
            .route("/room", web::post().to(new_room))
            .route("/rooms", web::get().to(get_rooms))
            .route("/room", web::delete().to(delete_room))
            .app_data(web::Data::new(client.clone()))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
