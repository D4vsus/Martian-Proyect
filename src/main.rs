mod net;
mod schemas;
mod model_manager;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![model,
                                       chat_completions,
                                       chat_completions_with_rag,
                                       embedding,
                                       learn])
}
