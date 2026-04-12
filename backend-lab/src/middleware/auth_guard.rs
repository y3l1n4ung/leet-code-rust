use axum::{
    http::Request,
    middleware::Next,
    response::Response,
};

pub async fn auth_guard(
    // TODO: Add an extractor to get the Authorization header
    // headers: HeaderMap,
    request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {

    // TODO: 
    // 1. Extract the "Bearer <token>" from the Authorization header.
    // 2. Decode the token using `jsonwebtoken::decode`.
    // 3. Return StatusCode::UNAUTHORIZED if invalid.
    // 4. Optionally, inject the decoded user ID into the request extensions for downstream handlers.

    todo!("Implement JWT validation middleware here.");

    // If successful, proceed to the next middleware or handler:
    // Ok(next.run(request).await)
}
