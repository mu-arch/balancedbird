use axum::routing::get;
use std::net::SocketAddr;
use axum::Router;
use axum::routing::post;
use tower_http::cors::{Any, CorsLayer};

#[path = "lib/types.rs"]
mod types;

#[path = "lib/helper.rs"]
mod helper;

#[tokio::main]
async fn main() {

    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow requests from any origin
        .allow_methods(Any) // Allow any HTTP method
        .allow_headers(Any);
    
    // Define web server routes
    let app = Router::new()
        .route("/", get(|| async { "Sup" }))
        .route("/weather/:code", get(helper::weather_handler)
            .layer(cors));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
