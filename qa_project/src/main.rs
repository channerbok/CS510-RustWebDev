/*
Credit to Axum Documentation
 https://docs.rs/axum/latest/axum/
 https://docs.rs/axum/latest/axum/extract/struct.State.html
 https://docs.rs/tower-http/0.5.2/tower_http/cors/index.html

 Credit to Tokio Documentation
 https://github.com/tokio-rs/axum

*/

use axum::http::{header, Method};
use std::net::SocketAddr;

use axum::body::Body;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::fmt;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct Store {
    questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(self::Store::init())),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}


#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
struct Question {
    id: i32,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}


#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
struct QuestionId(i32);




async fn handler_fallback() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}



// Handler to get questions
async fn get_questions(
    Query(params): Query<HashMap<String, String>>,
    State(store): State<Store>,
) -> Result<Response, MyError> {
    
    
    if let Some(n) = params.get("start") {
        println!("{:?}", n.parse::<usize>());
    }

    let questions = store.questions.read().await;
    let error_param = MyError::MissingParameters;

    let _start_param = match params.get("start") {
        Some(start) => match start.parse::<usize>() {
            Ok(start) => Some(start),
            Err(err) => return Err(MyError::ParseError(err)),
        },
        None => None,
    };

    // Error handle for missing parameters
    if questions.is_empty() {
        return Err(error_param);
    }

    // Return the json response
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(
            serde_json::to_string_pretty(&*questions).unwrap(),
        ))
        .unwrap();

    Ok(response)
}

// Custom Error type
#[allow(dead_code)]
#[derive(Debug)]
enum MyError {
    ParseError(std::num::ParseIntError),
    MissingParameters,
}

// Custom error type implementation
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
        }
    }
}


impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::ParseError(err) => write!(f, "Cannot parse parameter: {}", err),
            MyError::MissingParameters => write!(f, "Missing parameter"),
        }
    }
}


#[tokio::main]
async fn main() {
    let store = Store::new();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers([header::CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    let app = Router::new()
        .route("/questions", get(get_questions))
        .layer(cors)
        .with_state(store)
        .fallback(handler_fallback);

    let ip = SocketAddr::new([127, 0, 0, 1].into(), 3000);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    tracing::debug!("serving {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
