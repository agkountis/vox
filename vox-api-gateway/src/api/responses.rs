use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CompletionResponse {
    pub data: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct TtsResponse {
    pub audio: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct InteractionResponse {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
}
