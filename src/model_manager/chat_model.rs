use std::option::IterMut;
use llama_cpp::{LlamaModel, LlamaParams, LlamaSession, SessionParams};
use llama_cpp::standard_sampler::StandardSampler;
use once_cell::unsync::Lazy;
use serde::Serialize;
use serde_json::json;

const model_path: &str = "";
const MODEL: Lazy<LlamaModel> = Lazy::new(|| {
    LlamaModel::load_from_file("", LlamaParams::default())
        .expect("Failed to load MODEL")
});

pub struct chat_model {
    session : LlamaSession,
}

impl chat_model {
    fn new() -> Self {
        return Self {session: MODEL.create_session(SessionParams::default())
                            .unwrap()
        }
    }

    pub fn completition(&mut self, context: &str) -> impl Iterator<Item = String>{
        self.session.advance_context(context).unwrap();
        return self.session
            .start_completing_with(StandardSampler::default(), 1024)
            .unwrap()
            .into_strings()
            .into_iter();

    }

    pub fn model_info(&self) -> &str {
        return model_path;
    }
}

pub fn build() -> chat_model {
    return  chat_model::new();
}