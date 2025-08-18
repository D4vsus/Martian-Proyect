use rocket::serde::{Deserialize, Serialize, json::Json};

#[derive(Serialize, Deserialize)]
pub struct prediction_message {
    role:Option<String>,
    content:Option<String>
}