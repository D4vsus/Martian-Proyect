use rocket::serde::{Deserialize, Serialize, json::Json};

#[derive(Serialize, Deserialize)]
pub struct ModelInfo {
    info: String
}