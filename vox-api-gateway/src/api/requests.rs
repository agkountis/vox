use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct VoiceSettings {
    pub language_code: String,
    pub voice_name: String,
    pub voice_gender: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct AudioSettings {
    pub audio_encoding: String,
    pub speaking_rate: f64,
    pub pitch: f64,
    pub volume_gain_db: f64,
    pub sample_rate_hertz: i32,
    pub effects_profile_id: Vec<String>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct SpeechSettings {
    pub voice: Option<VoiceSettings>,

    pub audio: Option<AudioSettings>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InteractionRequest {
    pub prompt: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_settings: Option<SpeechSettings>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct TtsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<VoiceSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<AudioSettings>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct CompletionRequest {
    pub prompt: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}
