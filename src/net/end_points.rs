use std::fs::File;
use rocket::serde::json::Json;
use crate::schemas::prediction_message::prediction_message;
use crate::schemas::prediction_request::prediction_request;

#[get("/model")]
fn model() -> &'static str {
    "Hello, world!"
}

#[post("/chat/completions", data = "<request_content>")]
fn chat_completions(request_content:Json<prediction_request>) -> &'static str {
    "Hello, world!"
}

#[post("/chat/completions/with_rag", data = "<request_content>")]
fn chat_completions_with_rag(request_content:Json<prediction_request>) -> &'static str {
    "Hello, world!"
}

#[post("/embedding", data = "<request_content>")]
fn embedding(request_content:Json<prediction_request>) -> &'static str {
    "Hello, world!"
}

#[post("/learn", data = "<file>")]
fn learn(file:File) -> &'static str {
    "Hello, world!"
}