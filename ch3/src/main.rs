use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fmt,
    net::SocketAddr,
    sync::Arc,
};

use axum::body::Body;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct Store {
    questions: Arc<RwLock<Vec<Question>>>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
        }
    }

    fn init() -> Vec<Question> {
        let file = include_str!("../questions.json");
        match serde_json::from_str(file) {
            Ok(questions) => questions,
            Err(err) => {
                panic!("Can't read questions.json: {}", err);
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
struct Question {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub answer: String,
    pub tags: Option<HashSet<String>>,
    pub source: Option<String>,
}

async fn handler_fallback() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}

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

    if questions.is_empty() {
        return Err(error_param);
    }

    // Serialize each Question individually and join them into a single string
    let questions_json = questions
        .iter()
        .map(|question| serde_json::to_string_pretty(question).unwrap())
        .collect::<Vec<_>>()
        .join(",");

    let response_body = questions_json;


    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(response_body))
        .unwrap();

    Ok(response)
}



#[allow(dead_code)]
#[derive(Debug)]
enum MyError {
    ParseError(std::num::ParseIntError),
    MissingParameters,
}

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
        .allow_origin(vec!["http://127.0.0.1:3000".parse().unwrap()])
        .allow_headers(Any)
        .allow_methods(Any);

    let app = Router::new()
        .route("/api/v1/question", get(get_questions))
        .layer(cors)
        .with_state(store.clone()) // Clone the store for each route
        .fallback(handler_fallback);

    let ip = SocketAddr::new([127, 0, 0, 1].into(), 9000);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    tracing::debug!("serving {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
