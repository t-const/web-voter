use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub body: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub body: String,
    pub admin: String,
}
