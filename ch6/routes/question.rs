use axum::body::Body;
use axum::extract::Path;

use axum::Json;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use std::collections::HashMap;
use std::result::Result::Ok;
use serde_json::json;
use tokio::fs::File;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

use crate::types::pagination::MyError;
use crate::store::Store;
use crate::types::questions::{Question, QuestionId};


// Adds the the POST question to the json file
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

// Fallback
pub async fn handler_fallback() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}


// Pagination struct
#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}



#[allow(private_interfaces)]
// Formats pagination
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, MyError> {
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
pub async fn get_questions(
    Query(params): Query<HashMap<String, String>>,
    State(store): State<Store>,
) -> Result<Response, MyError> {
    log::info!("Start querying questions");
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
        log::info!("No pagination used");
        return Err(error_param);
    }

    // Return a set amount of questions based upon query parameters in request
    if !params.is_empty() {
        log::info!("Pagination set {:?}", &pagination);
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

    // Return the entire json response when pagination is not present in URL
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(
            serde_json::to_string_pretty(&*questions).unwrap(),
        ))
        .unwrap();

    Ok(response)
}


// Get a single question with using ID as a query parameter (http://localhost:3000/question/1)
pub async fn get_question(
    Path(id): Path<QuestionId>,
    State(store): State<Store>,
) -> Result<Response, MyError> {
    
    let store_clone = store.clone();
    let question = store_clone.get_question(id).await;

    let response = match question {
        Some(question) => {
            let body = serde_json::to_string(&question).unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .body(Body::from(body))
                .unwrap()
        }
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Question not found"))
            .unwrap(),
    };

    Ok(response)
}

// POST question
pub async fn add_question(
    State(store): State<Store>,
    Json(question): Json<Question>,
) -> Response<Body> {
    
    // Add to JSON
    let _temp = add_question_to_file(&question).await;

    // Add to hash map
    store.add_question_store(question).await;

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Question added"))
        .unwrap()
}


// Updates question, PUT implemenation
pub async fn update_question(
    State(store): State<Store>,
    Path(id): Path<QuestionId>,
    Json(question): Json<Question>,
) -> Result<Response, MyError> {
    
    

    match store.questions.write().await.get_mut(&id) {
        Some(q) => *q = question,
        None => return Err(MyError::QuestionNotFound),
    }


    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Question Updated"))
        .unwrap();

    Ok(response)
}



// Deletes question, DELETE implemenation
pub async fn delete_question(
    State(store): State<Store>,
   Path(id): Path<QuestionId>,
) -> Result<Response, MyError> {

    match store.questions.write().await.remove(&id) {
        Some(_) => {
            let response = Response::builder()
                .status(StatusCode::OK)
                .body(Body::from("Question Deleted"))
                .unwrap();
            Ok(response)
        }
        None => Err(MyError::QuestionNotFound),
    }
}