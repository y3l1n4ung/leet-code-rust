use axum::{
    response::sse::{Event, Sse},
    routing::get,
    Router,
};
use std::convert::Infallible;
use futures_util::stream::{self, Stream};

pub fn router() -> Router {
    Router::new()
        .route("/", get(sse_handler))
}

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // TODO: Create an actual channel (e.g., tokio::sync::broadcast) in your AppState
    // and subscribe to it here. Yield events as they arrive.
    
    let stream = stream::repeat_with(|| {
        Event::default().data("Server is alive!")
    });

    todo!("Replace this mock stream with a real tokio broadcast receiver stream.")
}
