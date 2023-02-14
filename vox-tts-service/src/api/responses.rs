use serde::{Deserialize, Serialize};

/// Audio byte stream encoded to base64.
#[derive(Default, Debug, Deserialize, Serialize)]
pub struct TtsResponse {
    pub audio: String,
}
