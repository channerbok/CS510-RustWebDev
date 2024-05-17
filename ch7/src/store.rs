use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;
use std::result::Result::Ok;


use crate::types::questions::NewQuestion;
use crate::types::pagination::MyError;
use crate::types::{
    answer::{Answer, AnswerId, NewAnswer},
    questions::{Question, QuestionId},
};
#[derive(Clone)]
pub struct Store {
    pub connection: PgPool,
}

// Connect PostgreSQL database to store class
impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection: {}", e),
        };
        Store {
            connection: db_pool,
        }
    }

    // Grabs all questions in database
    pub async fn get_questions(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<Question>, MyError> {
        match sqlx::query("SELECT * from questions LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(questions) => Ok(questions),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(MyError::DatabaseQueryError)
            }
        }
    }

    // Adds a new question to the database
    pub async fn add_question(&self, new_question: Question) -> Result<Question, sqlx::Error> {
        match sqlx::query(
            "INSERT INTO questions (title, content, tags) VALUES ($1, $2, $3)
                RETURNING id, title, content, tags",
        )
        .bind(new_question.title)
        .bind(new_question.content)
        .bind(new_question.tags)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => {
            // Print the error message if an error occurs
            println!("Error adding question: {}", e);
            Err(e)
            }
        }
    }

    // Updates a question in the data base
    pub async fn update_question(
        &self,
        question: Question,
        question_id: i32,
    ) -> Result<Question, sqlx::Error> {
        match sqlx::query(
            "UPDATE questions
                SET title = $1, content = $2, tags = $3
                WHERE id = $4
                RETURNING id, title, content, tags",
        )
        .bind(question.title)
        .bind(question.content)
        .bind(question.tags)
        .bind(question_id)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => Err(e),
        }
    }

    // Deletes question from database
    pub async fn delete_question(&self, question_id: i32) -> Result<bool, sqlx::Error> {
        match sqlx::query("DELETE FROM questions WHERE id = $1")
            .bind(question_id)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }

    // Adds answer to the data base by matching the answer id to the question id
    pub async fn add_answer(&self, new_answer: NewAnswer) -> Result<Answer, MyError> {
        match sqlx::query("INSERT INTO answers (content, question_id) VALUES ($1, $2)")
            .bind(new_answer.content)
            .bind(new_answer.question_id.0)
            .map(|row: PgRow| Answer {
                id: AnswerId(row.get("id")),
                content: row.get("content"),
                question_id: QuestionId(row.get("question_id")),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(answer) => Ok(answer),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(MyError::DatabaseQueryError)
            }
        }
    }
}
