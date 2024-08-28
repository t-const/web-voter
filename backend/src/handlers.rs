use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use tokio_postgres::Client;
use std::sync::Arc;
use crate::models::{User, Room, NewRoom};

#[derive(Deserialize)]
pub struct QueryParams {
    id: i32,
}

pub async fn get_user(client: web::Data<Arc<Client>>, params: web::Query<QueryParams>) -> impl Responder {
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

pub async fn get_users(client: web::Data<Arc<Client>>) -> impl Responder {
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

pub async fn get_room(client: web::Data<Arc<Client>>, params: web::Query<QueryParams>) -> impl Responder {
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

pub async fn new_room(client: web::Data<Arc<Client>>, new_room: web::Json<NewRoom>) -> impl Responder {
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

pub async fn get_rooms(client: web::Data<Arc<Client>>) -> impl Responder {
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

pub async fn delete_room(client: web::Data<Arc<Client>>, params: web::Query<QueryParams>) -> impl Responder {
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