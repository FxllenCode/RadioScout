use axum::{response::IntoResponse, routing::post, Router};

// Define the handler for the /hello route
async fn upload_handler() -> impl IntoResponse {
    "Hello, World!"
}

// Define a function to create the router for this route
pub fn upload_audio() -> Router {
    Router::new().route("/hello", post(upload_handler))
}