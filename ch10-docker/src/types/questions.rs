use serde::{Deserialize, Serialize};
use std::collections::HashSet;
// Question struct
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct QuestionResponse {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub answer: String,
    pub tags: Option<HashSet<String>>,
    pub source: String,
}


impl Serialize for QuestionId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash, sqlx::Type)]
pub struct QuestionId(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewQuestion {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}
