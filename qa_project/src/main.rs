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

use std::sync::RwLock;
use std::sync::Arc;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

use std::str::FromStr;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct Store {
 questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
}

impl Store{
    fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(self::Store::init())),
        }
    }

    /// Function to initialize the questions hashmap by reading in the questions from a json file
    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        let questions: HashMap<QuestionId, Question> =
            serde_json::from_str::<HashMap<QuestionId, Question>>(file)
                .unwrap()
                .into_iter()
                .collect();
        questions
    }

   
    
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
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



async fn handler_fallback() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}

#[derive(Debug)]
struct Pagination {
 start: usize,
 end: usize,
}



#[derive(Debug)]
struct InvalidId;

async fn get_questions(params: HashMap<String, String>, store: Arc<RwLock<HashMap<QuestionId, Question>>>) -> Result<Response<Json<Vec<Question>>>, StatusCode> {

    let guard = store.read().unwrap();


    let questions: Vec<Question> = guard.values().cloned().collect();

    Ok(Response::new(Json(questions)))
}



#[tokio::main]
async fn main() {
    let store = Store::new();
    
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers([header::CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    let app = Router::new()
       // .route("/questions", get(get_questions))
        .layer(cors)
        .with_state(store)
        .fallback(handler_fallback);
        

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
