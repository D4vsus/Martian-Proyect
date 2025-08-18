use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::yansi::Paint;
use serde_json::json;
use crate::model_manager;
use crate::model_manager::chat_model::{build, chat_model};
use crate::schemas::generate_respons::generate_respons;

pub fn model_info() -> Json<String>{

}

pub fn completion() -> Json<generate_respons> {
   let mut model: chat_model = build();
    let respons = generate_respons::new(String::from("default"));

    return Json(respons);

}