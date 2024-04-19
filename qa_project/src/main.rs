/*
Credit to Axum Documentation  - https://docs.rs/axum/latest/axum/

*/

use std::io::{Error, ErrorKind};
use std::str::FromStr;
use std::net::SocketAddr;
use axum::{
    routing::get,
    Router,
};

#[allow(dead_code)]
#[derive(Debug)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Debug)]
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



#[tokio::main]
async fn main() {
    
    // Create route with response
    let hello = Router::new()
    .route("/", get(|| async { "Hello, World!" }));
    
    // Bind tcplistener to localhost
    let listener = tokio::net::TcpListener::
    bind("0.0.0.0:3000").await.unwrap();
    
    // Handle incoming requests
    axum::serve(listener, hello).await.unwrap();
 
}
