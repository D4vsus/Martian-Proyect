mod net;
mod schemas;
mod model_manager;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    let config = Config {
        address: "0.0.0.0".parse().unwrap(),
        port: 8080,
        ..Config::debug_default()
    };

    rocket::build().mount("/", routes![model,
                                       chat_completions,
                                       chat_completions_with_rag,
                                       embedding,
                                       learn])
}
