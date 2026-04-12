use axum::{
    routing::post,
    Router, Json,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String, // Note: Always hash passwords in a real app!
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

pub fn router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}

async fn login(Json(payload): Json<LoginPayload>) -> Json<AuthResponse> {
    todo!("1. Query DB for user by username. 2. Verify password hash. 3. Create JWT using jsonwebtoken crate. 4. Return token.")
}

async fn register(Json(payload): Json<LoginPayload>) -> Json<AuthResponse> {
    todo!("1. Hash password. 2. Insert user into DB. 3. Generate JWT. 4. Return token.")
}
