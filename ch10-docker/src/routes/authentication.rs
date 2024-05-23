
use crate::types::account::Account;
use crate::types::account::AccountId;
use axum::body::Body;
use axum::Json;
use axum::{extract::State, http::StatusCode, response::Response};
use chrono::prelude::*;
use std::result::Result::Ok;
extern crate serde_json;
use crate::store::Store;
use crate::types::account::Session;
use crate::types::pagination::MyError;
use argon2::{self, Config};
use axum::response::IntoResponse;
use rand::Rng;

pub async fn register(
    State(store): State<Store>,
    Json(account): Json<Account>,
) -> Result<Response, MyError> {
    let hashed_password = hash_password(account.password.as_bytes());
    let account = Account {
        id: account.id,
        email: account.email,
        password: hashed_password,
    };

    if let Err(_e) = store.add_account(account).await {
        return Err(MyError::QuestionNotFound);
    }
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Account Added"))
        .unwrap();

    Ok(response)
}

pub fn hash_password(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

/*
pub async fn login(
    State(store): State<Store>,
    Json(login): Json<Account>,
) -> Result<Response, MyError> {
    let account_result = store.get_account(login.email).await;
    match account_result {
        Ok(account) => match verify_password(&account.password, login.password.as_bytes()) {
            Ok(verified) => {
                if verified {
                    let token = issue_token(account.id.expect("id not found"));
                    let json_response =
                        serde_json::to_string(&token).map_err(|_e| MyError::QuestionNotFound)?;
                    Ok((StatusCode::OK, json_response).into_response())
                } else {
                    Err(MyError::QuestionNotFound)
                }
            }
            Err(e) => Err(MyError::QuestionNotFound),
        },
        Err(e) => Err(e),
    }
}

fn verify_password(hash: &str, password: &[u8]) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}

fn issue_token(account_id: AccountId) -> String {
    let current_date_time = Utc::now();
    let dt = current_date_time + chrono::Duration::days(1);
    paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(&Vec::from("RANDOM WORDS WINTER MACINTOSH PC".as_bytes()))
        .set_expiration(&dt)
        .set_not_before(&Utc::now())
        .set_claim("account_id", serde_json::json!(account_id))
        .build()
        .expect("Failed to construct paseto token w/ builder!")
}

#[allow(clippy::needless_borrow)]
#[allow(dead_code)]
pub fn verify_token(token: String) -> Result<Session, MyError> {
    let token = paseto::tokens::validate_local_token(
        &token,
        None,
        &"RANDOM WORDS WINTER MACINTOSH PC".as_bytes(),
        &paseto::tokens::TimeBackend::Chrono,
    )
    .map_err(|_| MyError::QuestionNotFound)?;
    serde_json::from_value::<Session>(token).map_err(|_| MyError::QuestionNotFound)
}


*/