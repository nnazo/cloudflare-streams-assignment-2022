use axum::extract::Extension;
use axum::Server;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tower_http::trace::TraceLayer;

mod job;
use job::JobQueue;
mod error;
mod header;
mod routes;

#[tokio::main]
async fn main() {
    // Set the logging environment to debug if not set
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "job_queue_api=debug,tower_http=debug");
    }
    tracing_subscriber::fmt::init();
    // Create the routes and add in our queue data
    let routes = routes::routes()
        .layer(Extension(Arc::new(RwLock::new(JobQueue::new()))))
        .layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    // Start the server
    Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
