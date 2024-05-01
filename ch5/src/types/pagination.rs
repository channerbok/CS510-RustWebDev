use axum::body::Body;



use axum::{

    http::StatusCode,
    response::{IntoResponse, Response},
};


use std::collections::HashMap;
use std::result::Result::Ok;




#[derive(Debug)]
pub enum MyError {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
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
        }
    }
}

// Pagination struct
#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}


#[allow(dead_code)]
// Formats pagination
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, MyError> {
    if params.contains_key("start") && params.contains_key("end") {
        let start = match params.get("start").unwrap().parse::<usize>() {
            Ok(start) => start,
            Err(err) => return Err(MyError::ParseError(err)),
        };
        let end = match params.get("end").unwrap().parse::<usize>() {
            Ok(end) => end,
            Err(err) => return Err(MyError::ParseError(err)),
        };

        Ok(Pagination { start, end })
    } else {
        Err(MyError::MissingParameters)
    }
}