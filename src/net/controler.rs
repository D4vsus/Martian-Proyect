use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::yansi::Paint;
use serde_json::json;
use crate::model_manager;
use crate::model_manager::chat_model::{build, ChatModel};
use crate::model_manager::embedding_model::EmbeddingModel;
use crate::schemas::generate_respons::generate_respons;

pub fn model_info() -> Json<String>{
    let mut model: ChatModel = build();
}

pub fn completion() -> Json<generate_respons> {
   let mut model: ChatModel = build();
    let respons = generate_respons::new(String::from("default"));

    return Json(respons);
}

pub fn embedding() -> Json<generate_respons> {
   let mut model: EmbeddingModel = build();
    let respons = model.embedding(context);//TODO create DTO

    return Json(respons);
}