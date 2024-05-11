mod routes;
mod store;
mod types;

use crate::routes::answer::add_answer;
use crate::routes::question::add_question;
use crate::routes::question::delete_question;
use crate::routes::question::get_questions;
use crate::routes::question::handler_fallback;
use crate::routes::question::update_question;

use tracing_subscriber::fmt::format::FmtSpan;
use axum::http::{header, Method};
use axum::routing::{delete, post, put};
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // Logging
    let log_filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "practical_rust_book=info,warp=error".to_owned());

    // Traces the program
    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    // Creates a new store and connects it to the postgres database
    let store = store::Store::new("postgres://postgres:4411@localhost:3030/rustwebdev").await;

    // Executes the migration from store to database
    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .expect("Cannot run migration");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers([header::CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    let app = Router::new()
        .route("/questions", get(get_questions))
        .route("/questions", post(add_question))
        .route("/questions/:id", put(update_question))
        .route("/questions/:id", delete(delete_question))
        .route("/answer", post(add_answer))
        .layer(cors)
        .with_state(store)
        .fallback(handler_fallback);

    let ip = SocketAddr::new([127, 0, 0, 1].into(), 3000);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    tracing::debug!("serving {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}