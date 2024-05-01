use serde::{Deserialize, Serialize};
use axum::body::Body;

use axum::Json;
use axum::{
    extract::State,
    http::StatusCode,
    response::Response,
};




use crate::store::Store;
use crate::types::answer::Answer;



// POST question
pub async fn add_answer (
    State(store): State<Store>,
    Json(answer): Json<Answer>,
) -> Response<Body> {
    
    // Add to JSON
    store.add_answer_store(answer).await;

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Answer added"))
        .unwrap()
}