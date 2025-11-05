use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct InfoUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 3, message = "Username minimal harus 3 huruf"))]
    pub name: String,
    #[validate(email(message = "Invalid Email Format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password Anda terlalu pendek"))]
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    name: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct LoginUser {
    #[validate(email(message = "Invalid Email Format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password Anda terlalu pendek"))]
    pub password: String,
}

#[derive(Deserialize)]
pub struct CountUser {
    pub count: Option<i64>,
}

#[derive(Deserialize)]
pub struct Logout {
    pub session_id: String,
}

#[derive(Deserialize)]
pub struct Session {
    pub timeout: i64,
}
