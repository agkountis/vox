use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct CompletionRequest {
    pub prompt: String,
    pub max_tokens: Option<u16>,
    pub temperature: Option<f32>,
}
