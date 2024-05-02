use axum::body::Body;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use std::collections::HashMap;
use std::result::Result::Ok;

#[allow(dead_code)]
#[derive(Debug)]
pub enum MyError {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
    DatabaseQueryError,
}

// Custom error type implementation, converts to response
impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        match self {
            MyError::ParseError(_) => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Failed to parse integer"))
                .unwrap(),
            MyError::MissingParameters => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Missing parameters"))
                .unwrap(),
            MyError::QuestionNotFound => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Question Not Found"))
                .unwrap(),
            MyError::DatabaseQueryError => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Database Query Error"))
                .unwrap(),
        }
    }
}

// Pagination struct
#[derive(Debug, Default)]
pub struct Pagination {
    pub limit: Option<i32>,
    pub offset: i32,
}

#[allow(dead_code)]
// Formats pagination
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, MyError> {
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<i32>()
                    .map_err(MyError::ParseError)?,
            ),

            offset: params
                .get("offset")
                .unwrap()
                .parse::<i32>()
                .map_err(MyError::ParseError)?,
        });
    }
    Err(MyError::MissingParameters)
}
