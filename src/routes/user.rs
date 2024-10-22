use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{ Serialize, Deserialize };
use jsonwebtoken::{ encode, Header, EncodingKey };
use std::sync::Arc;
use crate::db::models::User;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String
}
