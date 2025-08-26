struct ConfigChatModel {
    models_path: Vec<&str>,
    default_model: Vec<&str>,
    n_gpu_layers: i32,
    n_threads: u32,
    max_tokens: i32,
    streaming: bool,
    n_ctx: u32,
}