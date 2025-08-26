struct ConfigRoot{
    let lang: &str,
    let memory_manager: ConfigMemoryManager,
    let server_manager: ConfigServerManager,
    let model_manager: ConfigModelManager,
    let initial_prompt: &str,
}