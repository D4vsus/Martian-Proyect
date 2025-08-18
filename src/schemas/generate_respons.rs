use rocket::serde::{Deserialize, Serialize, json::Json};

#[derive(Serialize, Deserialize)]
pub struct generate_respons{
    generated_text: String
}


impl generate_respons{
    pub fn new(generated_text:String) -> Self {
        return Self {generated_text: generated_text};
    }
}