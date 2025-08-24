use std::option::IterMut;
use llama_cpp::{CompletionHandle, LlamaContextError, LlamaModel, LlamaParams, LlamaSession, SessionParams, TokensToStrings};
use llama_cpp::standard_sampler::StandardSampler;
use once_cell::unsync::Lazy;
use serde::Serialize;
use serde_json::json;

const DEFAULT_MODEL_PATH: &str = "";
//TODO: turn CHAT_MODEL into CHAT_MODELS with HashMap
const CHAT_MODEL: Lazy<LlamaModel> = Lazy::new(|| {
    LlamaModel::load_from_file("", LlamaParams::default())
        .expect("Failed to load the model")
});

pub struct ChatModel {
    session : LlamaSession,
}

impl Chat_model {
    fn new() -> Self {
        return Self {session: MODEL.create_session(SessionParams::default())
                            .expect("ERROR: Unable to load model")
        }
    }

    pub fn completion(&mut self, context: &str) -> Result<impl Iterator<Item = String>, LlamaContextError>{
        self.session.advance_context(context)?;

        let completing:CompletionHandle = self.session.start_completing_with(StandardSampler::default(), 1024)?;
        let result: TokensToStrings<CompletionHandle> = completing.into_strings()
                    .into_iter();
        return Ok(result);
    }

    pub fn model_info(&self) -> &str {
        return model_path;
    }
}

pub fn build() -> chat_model {
    return  chat_model::new();
}