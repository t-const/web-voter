use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use env_logger;
use log::info;
use models::User;
use serde::Deserialize;
use std::env;
use std::sync::Arc;
use tokio_postgres::{Client, NoTls};

mod models;

#[derive(Deserialize)]
struct QueryParams {
    id: i32,
}

async fn get_user(client: web::Data<Arc<Client>>, params: web::Query<QueryParams>) -> impl Responder {
    let id = params.id;

    // Perform the query
    match client.query_one(
        "SELECT id, name, body, status FROM users WHERE id = $1",
        &[&id]
    ).await {
        Ok(row) => {
            // Map the result to the User struct
            let user = User {
                id: row.get(0),
                name: row.get(1),
                body: row.get(2),
                status: row.get(3),
            };
            HttpResponse::Ok().json(user) // Return the user as JSON
        }
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({ "error": "User not found" })),
    }
}

async fn test_handler() -> impl Responder {
    HttpResponse::Ok().json("This is a test")
}

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
            .route("/", web::get().to(test_handler))
            .app_data(web::Data::new(client.clone()))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
