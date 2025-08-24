use rocket::serde::{Deserialize, Serialize, json::Json};
use crate::schemas::prediction_message::PredictionMessage;

#[derive(Serialize, Deserialize)]
pub struct PredictionRequest {
    messages: Option<PredictionMessage>,
    max_tokens: Option<i64>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    stop: Option<Vec<String>>,
    model_selected: Option<String>
}