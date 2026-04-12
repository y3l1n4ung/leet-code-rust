mod routes;
mod state;
mod models;
mod middleware;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // Initialize tracing (logging)
    tracing_subscriber::fmt::init();

    // TODO: Initialize DB Pool and Redis Client from the `state` module.
    // let db_pool = state::db::init_db().await;
    // let redis_client = state::cache::init_redis().await;

    // TODO: Create a shared application state (e.g., AppState) and wrap it in std::sync::Arc.
    // let app_state = std::sync::Arc::new(AppState { db_pool, redis_client });

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(|| async { "Server is healthy!" }))
        // TODO: Merge routers from `routes::crud`, `routes::auth`, and `routes::sse` here.
        // .nest("/users", routes::crud::router())
        // .nest("/auth", routes::auth::router())
        // .nest("/events", routes::sse::router())
        // .with_state(app_state)
        ;

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
