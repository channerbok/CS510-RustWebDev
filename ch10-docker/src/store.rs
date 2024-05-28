use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;
use std::result::Result::Ok;

use crate::types::account::AccountId;
use crate::types::pagination::MyError;
use crate::types::questions::NewQuestion;

use crate::types::{
    account::Account,
    answer::{Answer, AnswerId, NewAnswer},
    questions::{Question, QuestionId},
};

#[derive(Clone)]
pub struct Store {
    pub connection: PgPool,
}

#[allow(dead_code)]
// Connect PostgreSQL database to store class
impl Store {
    pub async fn new() -> Self {
        let url: &str = "postgres://postgres:1234@db:5432/questions";
        let db_pool = match PgPoolOptions::new().max_connections(5).connect(url).await {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection: {}", e),
        };

        Store {
            connection: db_pool,
        }
    }

    pub async fn get_account(self, email: String) -> Result<Account, MyError> {
        match sqlx::query("SELECT * from accounts where email = $1")
            .bind(email)
            .map(|row: PgRow| Account {
                id: Some(AccountId(row.get("id"))),
                email: row.get("email"),
                password: row.get("password"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(account) => Ok(account),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(MyError::DatabaseQueryError)
            }
        }
    }

    pub async fn add_account(self, account: Account) -> Result<bool, MyError> {
        match sqlx::query(
            "INSERT INTO accounts (email, password)
            VALUES ($1, $2)",
        )
        .bind(account.email)
        .bind(account.password)
        .execute(&self.connection)
        .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(MyError::DatabaseQueryError)
            }
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

    // Grabs all questions in database
    pub async fn get_answers(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<Answer>, MyError> {
        match sqlx::query("SELECT * from answers LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Answer {
                id: AnswerId(row.get("id")),
                content: row.get("content"),
                question_id: QuestionId(row.get("corresponding_question")),
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
    pub async fn add_question(&self, new_question: NewQuestion) -> Result<Question, sqlx::Error> {

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
            Err(e) => Err(e),
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
        match sqlx::query("INSERT INTO answers (content, corresponding_question) VALUES ($1, $2) RETURNING id, content, corresponding_question")
            .bind(new_answer.content)
            .bind(new_answer.question_id.0)
            .map(|row: PgRow| Answer {
                id: AnswerId(row.get("id")),
                content: row.get("content"),
                question_id: QuestionId(row.get("corresponding_question")),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(answer) => Ok(answer),
            Err(_e) => {
                Err(MyError::DatabaseQueryError)
            }
        }
    }

    // Deletes answer from database
    pub async fn delete_answer(&self, question_id: i32) -> Result<bool, sqlx::Error> {
        match sqlx::query("DELETE FROM answers WHERE corresponding_question = $1")
            .bind(question_id)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }
}


