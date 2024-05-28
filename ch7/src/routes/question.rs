use crate::types::pagination::extract_pagination;
use crate::types::pagination::Pagination;
use axum::body::Body;
use axum::extract::Path;
use axum::Json;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::result::Result::Ok;
use tracing::info;
use tracing::{event, Level};
extern crate serde_json;
use std::collections::HashMap;

use crate::store::Store;
use crate::types::pagination::MyError;
use crate::types::questions::Question;

/// Handles when router find nothing
pub async fn handler_fallback() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}

// Handler to get questions
// Handles either query parameters in the request i.e. (http://localhost:3000/questions?offset=0&limit=5)
// Also handles the base line request and returns entire question json i.e. (http://localhost:3000/questions)
pub async fn get_questions(
    Query(params): Query<HashMap<String, String>>,
    State(store): State<Store>,
) -> Result<impl IntoResponse, MyError> {
    log::info!("Start querying questions");
    event!(target: "practical_rust_book", Level::INFO, "querying questions");

    let mut pagination = Pagination::default();

    // Return a set amount of questions based upon query parameters in request
    if !params.is_empty() {
        pagination = extract_pagination(params)?;
    }

    info!(pagination = false);
    log::info!("No pagination used");
    event!(target: "practical_rust_book", Level::INFO, "PAGE2");
    let res: Vec<Question> = match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => res,
        Err(_e) => return Err(MyError::DatabaseQueryError),
    };

    // Return the entire json response when pagination is not present in URL
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(serde_json::to_string_pretty(&*res).unwrap()))
        .unwrap();

    Ok(response)
}

// POST question
pub async fn add_question(
    State(store): State<Store>,
    Json(new_question): Json<Question>,
) -> Result<Response, MyError> {
    event!(target: "practical_rust_book", Level::INFO, "ADD");
    if let Err(_e) = store.add_question(new_question).await {
        return Err(MyError::QuestionNotFound);
    }
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Question Added"))
        .unwrap();

    Ok(response)
}

// Updates question, PUT implemenation
pub async fn update_question(
    Path(id): Path<i32>,
    State(store): State<Store>,
    Json(question): Json<Question>,
) -> Result<Response<Body>, MyError> {
    let res = match store.update_question(question, id).await {
        Ok(res) => res,
        Err(_e) => return Err(MyError::DatabaseQueryError),
    };

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(serde_json::to_string_pretty(&res).unwrap()))
        .unwrap();

    Ok(response)
}

// Deletes question, DELETE implemenation
pub async fn delete_question(
    Path(id): Path<i32>,
    State(store): State<Store>,
) -> Result<Response, MyError> {
    if let Err(_e) = store.delete_question(id).await {
        return Err(MyError::DatabaseQueryError);
    }

    if let Err(_e) = store.delete_answer(id).await {
        return Err(MyError::DatabaseQueryError);
    }

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(format!("Question {} deleted", id)))
        .unwrap();

    Ok(response)
}
