use axum::body::Body;
use axum::extract::Path;

use crate::types::pagination::extract_pagination;
use crate::types::pagination::Pagination;

use crate::types::questions::NewQuestion;

use axum::Json;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::result::Result::Ok;

extern crate serde_json;

use crate::store::Store;
use crate::types::pagination::MyError;

use crate::types::answer::NewAnswer;

pub async fn add_answer(
    State(store): State<Store>,
    Json(new_answer): Json<NewAnswer>,
) -> Result<Response, MyError> {
    if let Err(e) = store.add_answer(new_answer).await {
        return Err(MyError::DatabaseQueryError);
    }
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Answer Added"))
        .unwrap();

    Ok(response)
}
