/*
Credit to Axum Documentation  - https://docs.rs/axum/latest/axum/

*/
use axum::Json;
use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use hyper::server::conn::AddrIncoming;
use serde::Serialize;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use std::str::FromStr;

#[allow(dead_code)]
#[derive(Debug, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
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

async fn get_questions() -> impl axum::response::IntoResponse {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );

    Json(question)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/questions", get(get_questions));
    let addr = SocketAddr::new([127, 0, 0, 1].into(), 3000);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");
    axum::serve(listener, app).await.unwrap();
}

/*
let hello = warp::get()
 .map(|| format!("Hello, World!"));
 warp::serve(hello)
 .run(([127, 0, 0, 1], 1337))
 .await;
}
*/

/*
#[tokio::main]
async fn main() {

    // Create route with response
    let hello = Router::new()
    .route("/", get(|| async { "Hello, World!" }));

    // Bind tcplistener to localhost
    let listener = tokio::net::TcpListener::
    bind("0.0.0.0:3030").await.unwrap();

    // Handle incoming requests
    axum::serve(listener, hello).await.unwrap();

}
*/
