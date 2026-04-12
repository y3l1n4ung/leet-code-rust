use axum::{
    routing::{get, post, put, delete},
    Router, Json, extract::Path,
};
use serde::{Deserialize, Serialize};

// TODO: Define your input and output DTOs (Data Transfer Objects) here
#[derive(Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
}

pub fn router() -> Router {
    Router::new()
        .route("/", post(create_user).get(list_users))
        .route("/{id}", get(get_user).put(update_user).delete(delete_user))
}

// -----------------------------------------------------------------------------
// CRUD Handlers
// -----------------------------------------------------------------------------

async fn create_user(Json(payload): Json<CreateUserPayload>) -> Json<UserResponse> {
    todo!("1. Validate payload. 2. Insert into PostgreSQL using sqlx. 3. Return the created UserResponse.")
}

async fn list_users() -> Json<Vec<UserResponse>> {
    todo!("1. Query PostgreSQL for all users. 2. Return the list.")
}

async fn get_user(Path(id): Path<uuid::Uuid>) -> Json<UserResponse> {
    todo!("1. Check Redis cache for user {id}. 2. If miss, query PostgreSQL. 3. Set Redis cache. 4. Return user.")
}

async fn update_user(Path(id): Path<uuid::Uuid>, Json(payload): Json<CreateUserPayload>) -> Json<UserResponse> {
    todo!("1. Update user in PostgreSQL. 2. Invalidate/update Redis cache. 3. Return updated user.")
}

async fn delete_user(Path(id): Path<uuid::Uuid>) {
    todo!("1. Delete user from PostgreSQL. 2. Delete user from Redis cache.")
}
