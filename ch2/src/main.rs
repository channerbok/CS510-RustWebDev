use std::io::{Error, ErrorKind};
use std::str::FromStr;

// Question struct that serves as the quetion and its contents
#[allow(dead_code)]
#[derive(Debug)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

// Question ID that is used to uniquely identify the questions from each other
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

// Implements the FromStr trait for QuestionId and allows parsing of strings into QuestionId
// If the string is not empty, it creates a new QuestionId Otherwise, it returns an error
impl FromStr for QuestionId {
    type Err = std::io::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

// Created new question and prints it
fn main() {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );
    println!("{:?}", question);
}
