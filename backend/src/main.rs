use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use env_logger;
use log::info;
use models::{User, Room, NewRoom};
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

async fn get_users(client: web::Data<Arc<Client>>) -> impl Responder {
    // Perform the query
    match client.query(
        "SELECT id, name, body, status FROM users",
        &[]
    ).await {
        Ok(rows) => {
            let users: Vec<User> = rows.iter().map(|row| User {
                id: row.get(0),
                name: row.get(1),
                body: row.get(2),
                status: row.get(3),
            }).collect();            
            HttpResponse::Ok().json(users) // Return the user as JSON
        }
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({ "error": "Users not found" })),
    }
}

async fn get_room(client: web::Data<Arc<Client>>, params: web::Query<QueryParams>) -> impl Responder {
    let id = params.id;

    // Perform the query
    match client.query_one(
        "SELECT id, name, body, admin FROM rooms WHERE id = $1",
        &[&id]
    ).await {
        Ok(row) => {
            // Map the result to the User struct
            let room = Room {
                id: row.get(0),
                name: row.get(1),
                body: row.get(2),
                admin: row.get(3),
            };
            HttpResponse::Ok().json(room)
        }
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({ "error": "Room not found" })),
    }
}

async fn new_room(client: web::Data<Arc<Client>>, new_room: web::Json<NewRoom>) -> impl Responder {
    let new_room = new_room.into_inner();

    // Execute the INSERT query
    match client.execute(
        "INSERT INTO rooms (name, body, admin) VALUES ($1, $2, $3)",
        &[&new_room.name, &new_room.body, &new_room.admin]
    ).await {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({ "status": "Room created" })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": "Failed to create room" })),
    }
}

async fn get_rooms(client: web::Data<Arc<Client>>) -> impl Responder {
    // Perform the query
    match client.query(
        "SELECT id, name, body, admin FROM rooms",
        &[]
    ).await {
        Ok(rows) => {
            let rooms: Vec<Room> = rows.iter().map(|row| Room {
                id: row.get(0),
                name: row.get(1),
                body: row.get(2),
                admin: row.get(3),
            }).collect();            
            HttpResponse::Ok().json(rooms) // Return the user as JSON
        }
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({ "error": "Rooms not found" })),
    }
}

async fn delete_room(client: web::Data<Arc<Client>>, params: web::Query<QueryParams>) -> impl Responder {
    let id = params.id;

    // Execute the DELETE query
    match client.execute(
        "DELETE FROM rooms WHERE id = $1",
        &[&id]
    ).await {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                HttpResponse::Ok().json(serde_json::json!({ "status": "Room deleted" }))
            } else {
                HttpResponse::NotFound().json(serde_json::json!({ "error": "Room not found" }))
            }
        }
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": "Failed to delete room" })),
    }
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
