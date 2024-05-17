use axum::body::Body;
use axum::extract::Path;

use crate::types::pagination::extract_pagination;
use crate::types::pagination::Pagination;
use axum::response::Html;
use crate::types::questions::NewQuestion;

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
// Handles either query parameters in the request i.e. (http://localhost:3000/questions?limit=0&offset=5)
// Also handles the base line request and returns entire question json i.e. (http://localhost:3000/questions)
pub async fn get_questions(
    Query(params): Query<HashMap<String, String>>,
    State(store): State<Store>,
) -> Result<impl IntoResponse, MyError> {
    
    let mut pagination = Pagination::default();

    // Return a set amount of questions based upon query parameters in request
    if !params.is_empty() {
        log::info!("Pagination set {:?}", &pagination);
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    }

    info!(pagination = false);
    log::info!("No pagination used");

     let questions_result = store.get_questions(pagination.limit, pagination.offset).await;
    
    match questions_result {
        Ok(questions) => {

            let mut html_string = String::from("<html><head><title>Questions</title></head><body><h1>Questions</h1><ul>");
            
            for question in questions {
                let tags_str = match &question.tags {
                    Some(tags) => tags.join(", "),
                    None => String::from("No tags"),
                };
                html_string.push_str(&format!(
                    "<li><h2>{}</h2><p>{}</p><p>Question ID: {}</p><p>Tags: {}</p></li>",
                    question.title,
                    question.content,
                    question.id.0,
                    tags_str,
                ));
            }

            html_string.push_str("</ul></body></html>");

            Ok(Html(html_string))
        }
        Err(_e) => {
            Err(MyError::DatabaseQueryError)
        }
    }
     
}



// POST question
pub async fn add_question(
    State(store): State<Store>,
    Json(new_question): Json<NewQuestion>,
) -> Result<Response, MyError> {
    
     log::info!("ADD");
    event!(target: "practical_rust_book", Level::INFO, "ADD");
    if let Err(_e) = store.add_question(new_question).await {
        return Err(MyError::DatabaseQueryError);
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

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(format!("Question {} Added", id)))
        .unwrap();

    Ok(response)
}
