use tokio;
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = "host=localhost user=admin password=admin dbname=postgres";
    let (client, connection) = tokio_postgres::connect(config, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Example: Create a table
    client.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            data    BYTEA
        )",
        &[],
    ).await?;

    // Example: Insert data
    client.execute(
        "INSERT INTO person (name, data) VALUES ($1, $2)",
        &[&"John Doe", &None::<&[u8]>],
    ).await?;

    // Example: Query data
    let rows = client.query("SELECT id, name, data FROM person", &[]).await?;

    for row in rows {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let _data: Option<&[u8]> = row.get(2);

        println!("Found person: {} with id: {}", name, id);
    }

    Ok(())
}
