use std::option::IterMut;
use llama_cpp::{CompletionHandle, EmbeddingsParams, LlamaContextError, LlamaModel, LlamaParams, LlamaSession, SessionParams, TokensToStrings};
use llama_cpp::standard_sampler::StandardSampler;
use once_cell::unsync::Lazy;
use serde::Serialize;
use serde_json::json;

const MODEL_PATH: &str = "";
const EMBEDDING_MODEL: Lazy<LlamaModel> = Lazy::new(|| {
    LlamaModel::load_from_file("", LlamaParams::default())
        .expect("Failed to load MODEL")
});

pub struct EmbeddingModel {}

impl EmbeddingModel {
    fn new() -> Self {
        return Self {}
    }

    pub fn embedding(&mut self, context: &str) -> Result<Vec<Vec<f32>>, LlamaContextError> {
        return Ok(EMBEDDING_MODEL.embeddings(inputs, EmbeddingsParams::default()));
    }

    pub fn embedding_len() -> usize{
        return EMBEDDING_MODEL.embed_len();
    }

    pub fn model_info(&self) -> &str {
        return MODEL_PATH;
    }
}