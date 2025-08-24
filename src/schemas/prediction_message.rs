use rocket::serde::{Deserialize, Serialize, json::Json};

#[derive(Serialize, Deserialize)]
pub struct PredictionMessage {
    role:Option<String>,
    content:Option<String>
}