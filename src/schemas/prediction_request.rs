use rocket::serde::{Deserialize, Serialize, json::Json};
use crate::schemas::prediction_message::prediction_message;

#[derive(Serialize, Deserialize)]
pub struct prediction_request {
    messages: Option<prediction_message>,
    max_tokens: Option<i64>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    stop: Option<Vec<String>>,
    model_selected: Option<String>
}