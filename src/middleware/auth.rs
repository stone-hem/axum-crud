// src/middleware/auth.rs
use axum::{
    extract::{State, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
    body::Body, // Important to import body
};
use sqlx::postgres::PgPool;
use reqwest;

pub async fn auth_middleware(
    State(pool): State<PgPool>,
    req: Request<Body>, // Ensure Request<Body> is used
    next: Next, // Ensure Next<Body> is used
) -> Result<Response<Body>, (StatusCode, String)> {
    let headers = req.headers();
    let auth_header = headers.get("Authorization");

    match auth_header {
        Some(token) => {
            let token_str = token.to_str().unwrap().trim_start_matches("Bearer ").to_string();
            match check_token(&pool, &token_str).await {
                Ok(true) => Ok(next.run(req).await),
                Ok(false) => Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string())),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
            }
        }
        None => Err((StatusCode::UNAUTHORIZED, "Authorization header missing".to_string())),
    }
}

async fn check_token(pool: &PgPool, token: &str) -> Result<bool, String> {
    let client = reqwest::Client::new();
    let url = format!("http://127.0.0.1:8000/api/v1/api:XLmsV25/user");

    let response = client.get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status() == StatusCode::OK {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(e) => Err(format!("Error checking token: {}", e)),
    }
}