use axum::body::Body;

use axum::Json;
use axum::{extract::State, http::StatusCode, response::Response};
use std::result::Result::Ok;

extern crate serde_json;

use crate::store::Store;
use crate::types::pagination::MyError;

use crate::types::answer::NewAnswer;


// Adds answer to the database
// _e was still flagging clippy so I added allow clause
#[allow(unused_variables)]
pub async fn add_answer(
    State(store): State<Store>,
    Json(new_answer): Json<NewAnswer>,
) -> Result<Response, MyError> {
    if let Err(_e) = store.add_answer(new_answer).await {
        return Err(MyError::DatabaseQueryError);
    }
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Answer Added"))
        .unwrap();

    Ok(response)
}
