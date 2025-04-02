use axum::{
    routing::{get, post, put, delete},
    Router,
    serve,
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer, Any};

use crate::routes::users::{create_user, get_user, update_user, delete_user, get_all_users};
use crate::middleware::auth::auth_middleware;

mod config;
mod routes;
mod models;
mod repos;
mod middleware;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let app = Router::new()
        .route("/users", get(get_all_users).post(create_user))
        .route("/users/{id}", get(get_user).put(update_user).delete(delete_user))
        .layer(axum::middleware::from_fn_with_state(pool.clone(), auth_middleware))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await?;
    println!("Server running on {}", addr);

    serve(listener, app).await?;

    Ok(())
}
