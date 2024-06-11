use std::collections::HashSet;
use std::str::FromStr;

// Question struct that serves as the question and its contents
#[derive(Debug)]
pub struct QuestionStruct {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub answer: String,
    pub tags: Option<HashSet<String>>,
    pub source: Option<String>,
}

// Question ID that is used to uniquely identify the questions from each other
#[derive(Debug)]
pub struct QuestionId(i32);

impl QuestionStruct {
    pub fn new(
        id: i32,
        title: String,
        content: String,
        answer: String,
        tags: Option<HashSet<String>>,
        source: Option<String>,
    ) -> Self {
        QuestionStruct {
            id: QuestionId(id),
            title,
            content,
            answer,
            tags,
            source,
        }
    }
}

// Implements the FromStr trait for QuestionId and allows parsing of strings into QuestionId
// If the string is not empty, it creates a new QuestionId Otherwise, it returns an error
impl FromStr for QuestionId {
    type Err = std::io::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.parse::<i32>() {
            Ok(parsed_id) => Ok(QuestionId(parsed_id)),
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid ID",
            )),
        }
    }
}

// Created new question and prints it
fn main() {
    let question = QuestionStruct::new(
        1,
        "First Question".to_string(),
        "Content of question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()].into_iter().collect()),
        Some("Some source".to_string()),
    );
    println!("{:?}", question);
}
