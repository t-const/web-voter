use tokio_postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub body: String,
    pub status: String,
}

pub async fn get_user_by_id(client: &Client, user_id: i32) -> Result<User, tokio_postgres::Error> {
    let row = client
        .query_one(
            "SELECT id, name, body, status FROM users WHERE id = $1",
            &[&(user_id)],
        )
        .await?;

    Ok(User {
        id: row.get("id"),
        name: row.get("name"),
        body: row.get("body"),
        status: row.get("status"),
    })
}
