// src/repositories/user_repository.rs
use sqlx::{postgres::PgPool, Error};
use crate::models::user::{User, NewUser, UpdateUser};
use chrono::Utc;


pub async fn create_user_db(pool: &PgPool, new_user: NewUser) -> Result<User, Error> {
    let now = Utc::now().naive_utc(); // Convert to NaiveDateTime
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (first_name, last_name, calling_code, phone, password, created_at, updated_at, emailisverified, phoneisverified, first_time_login, is_active, is_organization_admin) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, false, false, true, true, false) 
        RETURNING id, role_id, first_name, last_name, calling_code, phone, about_me, designation, email, emailisverified, phoneisverified, email_verified_at, phone_verified_at, photo, password, first_time_login, is_active, organization_id, is_organization_admin, created_at, updated_at",
        new_user.first_name,
        new_user.last_name,
        new_user.calling_code,
        new_user.phone,
        new_user.password,
        now,
        now
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_db(pool: &PgPool, id: i64) -> Result<User, Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, role_id, first_name, last_name, calling_code, phone, about_me, designation, email, emailisverified, phoneisverified, email_verified_at, phone_verified_at, photo, password, first_time_login, is_active, organization_id, is_organization_admin, created_at, updated_at FROM users WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(user)
}

pub async fn update_user_db(pool: &PgPool, id: i64, update_user: UpdateUser) -> Result<User, Error> {
    let now = Utc::now().naive_utc(); // Convert to NaiveDateTime
    let user = sqlx::query_as!(
        User,
        "UPDATE users SET 
            role_id = COALESCE($1, role_id),
            first_name = COALESCE($2, first_name),
            last_name = COALESCE($3, last_name),
            calling_code = COALESCE($4, calling_code),
            phone = COALESCE($5, phone),
            about_me = COALESCE($6, about_me),
            designation = COALESCE($7, designation),
            email = COALESCE($8, email),
            photo = COALESCE($9, photo),
            password = COALESCE($10, password),
            organization_id = COALESCE($11, organization_id),
            is_organization_admin = COALESCE($12, is_organization_admin),
            updated_at = $13 
        WHERE id = $14 
        RETURNING id, role_id, first_name, last_name, calling_code, phone, about_me, designation, email, emailisverified, phoneisverified, email_verified_at, phone_verified_at, photo, password, first_time_login, is_active, organization_id, is_organization_admin, created_at, updated_at",
        update_user.role_id,
        update_user.first_name, 
        update_user.last_name,
        update_user.calling_code,
        update_user.phone,
        update_user.about_me,
        update_user.designation,
        update_user.email,
        update_user.photo,
        update_user.password,
        update_user.organization_id,
        update_user.is_organization_admin,
        now,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(user)
}

pub async fn delete_user_db(pool: &PgPool, id: i64) -> Result<(), Error> {
    sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_all_users_db(pool: &PgPool) -> Result<Vec<User>, Error> {
    let users = sqlx::query_as!(
        User,
        "SELECT id, role_id, first_name, last_name, calling_code, phone, about_me, designation, email, emailisverified, phoneisverified, email_verified_at, phone_verified_at, photo, password, first_time_login, is_active, organization_id, is_organization_admin, created_at, updated_at FROM users"
    )
    .fetch_all(pool)
    .await?;
    Ok(users)
}