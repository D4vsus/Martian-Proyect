struct ConfigMemoryManager{
    manger: &str,
    minimum_similarity: f32,
    maximum_context_return: u32,
    chunk_size: u32,
    chunk_overlap: u32,
}