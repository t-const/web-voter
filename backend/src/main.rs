use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

mod postgres;

async fn home() -> impl Responder {
    HttpResponse::Ok().body("PostgreSQL connected")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug");
    let host = env::var("POSTGRES_HOSTNAME").expect("Host not set");
    let port = env::var("POSTGRES_PORT").expect("Port not set");

    let pool = postgres::get_pool();
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .route("/", web::get().to(home))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
