

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
use std::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

// Thus stores our hashmap of questions and can be passed around the program
// Mock database type
#[derive(Clone)]
struct Store {
    // Question hashmap
    questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
}

impl Store {
    // Creates a new instance of Store
    fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(self::Store::init())),
        }
    }
    // Initializes the questions hashmap from a JSON file
    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}

// Question struct that serves as the quetion and its contents
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
struct Question {
    id: i32,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
struct QuestionId(i32);

// Fall back in case our route fails to find anything
async fn handler_fallback() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}

// Handler to get all questions from the json file
// Also Uuses pagination
async fn get_questions(
    Query(params): Query<HashMap<String, String>>,
    State(store): State<Store>,
) -> Result<Response, MyError> {
    if let Some(n) = params.get("start") {
        println!("{:?}", n.parse::<usize>());
    }

    // Reads in question
    let questions = store.questions.read().await;
    let error_param = MyError::MissingParameters;

    // Error handle for parser error
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


// Error displaying
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::ParseError(err) => write!(f, "Cannot parse parameter: {}", err),
            MyError::MissingParameters => write!(f, "Missing parameter"),
        }
    }
}


// Creates one route for the questions to be displayed.
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
