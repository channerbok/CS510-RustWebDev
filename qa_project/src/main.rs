/*
Credit to Axum Documentation
 https://docs.rs/axum/latest/axum/
 https://docs.rs/axum/latest/axum/extract/struct.State.html
 https://docs.rs/tower-http/0.5.2/tower_http/cors/index.html

 Credit to Tokio Documentation
 https://github.com/tokio-rs/axum

*/

use axum::body::Body;
use axum::http::{header, Method};
use axum::routing::{post, put};
use axum::Json;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fmt;
use std::net::SocketAddr;
use std::result::Result::Ok;
use std::sync::Arc;
use tokio::fs::File;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;
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

    // Adds question to hashmap
    async fn add_question_store(self, question: Question) -> Self {
        self.questions
            .write()
            .await
            .insert(question.id.clone(), question);
        self
    }

    // Updates question to hashmap
    async fn update_question(self, question: Question) -> Self {
        match self.questions.write().await.get_mut(&question.id) {
            Some(q) => *q = question,
            None => {}
        }
        self
    }
}

// Adds the the POST question to the json
async fn add_question_to_file(question: &Question) -> tokio::io::Result<File> {
    let file_path = "../questions.json";

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .await?;

    let json_question = json!(question);

    let json_string = serde_json::to_string(&json_question)?;

    file.write_all(json_string.as_bytes()).await?;

    Ok(file)
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
struct QuestionId(i32);

async fn handler_fallback() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}

#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

// Formats pagination
fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, MyError> {
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

// Handler to get questions
// Handles either query parameters in the request i.e. (http://localhost:3000/questions?start=0&end=5)
// Also handles the base line request and returns entire question json i.e. (http://localhost:3000/questions)
async fn get_questions(
    Query(params): Query<HashMap<String, String>>,
    State(store): State<Store>,
) -> Result<Response, MyError> {
    let questions = store.questions.read().await;
    let error_param = MyError::MissingParameters;

    // Error handle for parser
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

    // Return a set amount of questions based upon query parameters in request
    if !params.is_empty() {
        let pagination = extract_pagination(params.clone()).unwrap();
        let json_string = serde_json::to_string_pretty(&*questions).unwrap();
        let start_index = pagination.start;
        let end_index = pagination.end.min(json_string.len());
        let questions: HashMap<String, Question> = serde_json::from_str(&json_string).unwrap();

        let mut sliced_questions: HashMap<String, Question> = HashMap::new();
        for (key, value) in questions.iter().skip(start_index).take(end_index) {
            sliced_questions.insert(key.clone(), value.clone());
        }

        let sliced_json_string = serde_json::to_string_pretty(&sliced_questions).unwrap();

        let response = Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(sliced_json_string))
            .unwrap();

        return Ok(response);
    }

    // Return the entire json response
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(
            serde_json::to_string_pretty(&*questions).unwrap(),
        ))
        .unwrap();

    Ok(response)
}

// POST question
async fn add_question(
    State(store): State<Store>,
    Json(question): Json<Question>,
) -> Response<Body> {
    // Add tohash map
    let _temp = add_question_to_file(&question).await;

    // Add to JSON
    store.add_question_store(question).await;

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Question added"))
        .unwrap()
}

// Updates question, PUT implemenation 
async fn update_question(
    State(store): State<Store>,
    Json(question): Json<Question>,
) -> Result<Response, MyError> {
    let cloned_question = question.clone();
    match store.questions.write().await.get_mut(&question.id) {
        Some(q) => *q = question,
        None => return Err(MyError::QuestionNotFound),
    }

    // Updates Hash Map
    store.update_question(cloned_question).await;

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Question Updated"))
        .unwrap();

    Ok(response)
}

// Custom Error type
#[allow(dead_code)]
#[derive(Debug)]
enum MyError {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
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
            MyError::QuestionNotFound => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Question Not Found"))
                .unwrap(),
        }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::ParseError(err) => write!(f, "Cannot parse parameter: {}", err),
            MyError::MissingParameters => write!(f, "Missing parameter"),
            MyError::QuestionNotFound => write!(f, "Question Not Found"),
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
        .route("/questions", post(add_question))
        .route("/questions/:id", put(update_question))
        .layer(cors)
        .with_state(store)
        .fallback(handler_fallback);

    let ip = SocketAddr::new([127, 0, 0, 1].into(), 3000);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    tracing::debug!("serving {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
