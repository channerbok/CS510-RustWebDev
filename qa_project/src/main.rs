/*
Credit to Axum Documentation  - https://docs.rs/axum/latest/axum/

*/

use axum::http::{header, Method};

use axum::Json;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};



use serde::Serialize;


use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use std::str::FromStr;

use tower_http::cors::{Any, CorsLayer};

#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
struct QuestionId(String);
impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

fn error_check(question: Question) -> MyError {
    if question.title.is_empty() {
        return MyError::MissingParameter;
    }

    if question.content.is_empty() {
        return MyError::MissingParameter;
    }

    let _open_vector = match question.tags {
        Some(inner_vec) => !inner_vec.is_empty() && inner_vec.iter().any(|s| !s.is_empty()),
        None => false,
    };

    if let Err(_) = question.id.0.parse::<i32>() {

        return MyError::InvalidId;
    }

    MyError::NoError
}

enum MyError {
    MissingParameter,
    NoError,
    InvalidId,
}

#[derive(Debug)]
struct InvalidId;

async fn get_questions() -> Result<Json<Question>, MyError> {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );

    let error = error_check(question.clone());

    match error {
        MyError::MissingParameter => {
            Err(MyError::MissingParameter)
        }
        MyError::InvalidId => {
            Err(MyError::InvalidId)
        }
        MyError::NoError => Ok(Json(question)),
    }
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let body = match self {
            MyError::MissingParameter => "Missing Parameter",
            MyError::InvalidId => "Invalid ID2",
            MyError::NoError => "No Error",
        };

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers([header::CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    let app = Router::new()
        .route("/questions", get(get_questions))
        .layer(cors);

    let addr = SocketAddr::new([127, 0, 0, 1].into(), 3000);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app).await.unwrap();
}
