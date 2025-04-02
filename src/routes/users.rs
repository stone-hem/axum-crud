// src/routes/users.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::postgres::PgPool;
use crate::models::user::{User, NewUser, UpdateUser};
use crate::repos::user_repository::{create_user_db, get_user_db, update_user_db, delete_user_db, get_all_users_db};

pub async fn create_user(State(pool): State<PgPool>, Json(new_user): Json<NewUser>) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    match create_user_db(&pool, new_user).await {
        Ok(user) => Ok((StatusCode::CREATED, Json(user))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_user(Path(id): Path<i32>, State(pool): State<PgPool>) -> Result<Json<User>, (StatusCode, String)> {
    match get_user_db(&pool, id as i64).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}

pub async fn update_user(Path(id): Path<i32>, State(pool): State<PgPool>, Json(update_user): Json<UpdateUser>) -> Result<Json<User>, (StatusCode, String)> {
    match update_user_db(&pool, id as i64, update_user).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn delete_user(Path(id): Path<i32>, State(pool): State<PgPool>) -> Result<StatusCode, (StatusCode, String)> {
    match delete_user_db(&pool, id as i64).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_all_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    match get_all_users_db(&pool).await {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}