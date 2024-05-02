use serde_json::json;

use std::collections::HashMap;

use std::result::Result::Ok;
use std::sync::Arc;

use tokio::fs::File;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;

use crate::types::{
    answer::{Answer, AnswerId},
    questions::{Question, QuestionId},
};

// Mock Data base type
#[derive(Clone)]
pub struct Store {
    pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
}

// Initialize data base
// Functions to delete/change data base
impl Store {
    pub fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Store::init())),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // Gets question from hashmap
    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }

    // Adds question to hashmap
    pub async fn get_question(&self, id: QuestionId) -> Option<Question> {
        let questions = self.questions.read().await;
        questions.get(&id).cloned()
    }

    // Adds quesiton to hashmap
    pub async fn add_question_store(&self, question: Question) {
        self.questions
            .write()
            .await
            .insert(question.id.clone(), question);
    }

    // Adds answer to hashmap
    pub async fn add_answer_store(&self, answer: Answer) {
        self.answers.write().await.insert(answer.id.clone(), answer);
    }
}

// Adds the the POST question to the json file
#[allow(dead_code)]
pub async fn add_question_to_file(question: &Question) -> tokio::io::Result<File> {
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
