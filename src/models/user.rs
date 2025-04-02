// src/models/user.rs
use serde::{Deserialize, Serialize};
use chrono::naive::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub role_id: Option<i64>,
    pub first_name: String,
    pub last_name: String,
    pub calling_code: i32,
    pub phone: i32,
    pub about_me: Option<String>,
    pub designation: Option<String>,
    pub email: Option<String>,
    pub emailisverified: bool,
    pub phoneisverified: bool,
    pub email_verified_at: Option<NaiveDateTime>, // Change type to NaiveDateTime
    pub phone_verified_at: Option<NaiveDateTime>, // Change type to NaiveDateTime
    pub photo: Option<String>,
    pub password: String,
    pub first_time_login: bool,
    pub is_active: bool,
    pub organization_id: Option<i64>,
    pub is_organization_admin: bool,
    pub created_at: Option<NaiveDateTime>, // Change type to NaiveDateTime
    pub updated_at: Option<NaiveDateTime>, // Change type to NaiveDateTime
}

#[derive(Deserialize)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub calling_code: i32,
    pub phone: i32,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub role_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub calling_code: Option<i32>,
    pub phone: Option<i32>,
    pub about_me: Option<String>,
    pub designation: Option<String>,
    pub email: Option<String>,
    pub photo: Option<String>,
    pub password: Option<String>,
    pub organization_id: Option<i64>,
    pub is_organization_admin: Option<bool>,
}