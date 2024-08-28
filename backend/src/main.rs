use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::{Client, NoTls};

mod db;
use db::get_user_by_id;

async fn get_user_by_id_handler(client: web::Data<Mutex<Client>>, path: web::Path<i32>) -> impl Responder {
    let user_id = path.into_inner();
    let client = client.lock().await;
    match get_user_by_id(&client, user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load the env var from the .env file

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to the db
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .expect("Failed to connect to the database");

    // Wrap the client in Arc and Mutex
    let client = Arc::new(Mutex::new(client));

    // Spawn a separate task to drive the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone())) // Share DB client with handlers
            .wrap(
                Cors::default()
                    .allow_any_origin() // Allow any origin for development. Use caution with this in production.
                    .allow_any_method()
                    .allow_any_header(),
            )
            .route("/users/{id}", web::get().to(get_user_by_id_handler))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
