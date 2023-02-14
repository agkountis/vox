use serde::{Deserialize, Serialize};
use vox_tts::{
    AudioConfig, AudioEncoding, InputSource, SsmlVoiceGender, SynthesisInput,
    SynthesizeSpeechRequest, VoiceSelectionParams,
};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct VoiceSettings {
    pub language_code: String,
    pub voice_name: String,
    pub voice_gender: String,
}

fn derive_gender_code(gender: &str) -> i32 {
    match gender {
        "male" => SsmlVoiceGender::Male as i32,
        "female" => SsmlVoiceGender::Female as i32,
        _ => SsmlVoiceGender::Unspecified as i32,
    }
}

impl From<VoiceSettings> for VoiceSelectionParams {
    fn from(value: VoiceSettings) -> Self {
        Self {
            language_code: value.language_code,
            name: value.voice_name,
            ssml_gender: derive_gender_code(value.voice_gender.as_str()),
        }
    }
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

impl From<AudioSettings> for AudioConfig {
    fn from(value: AudioSettings) -> Self {
        let encoding = match value.audio_encoding.as_str() {
            "mp3" => AudioEncoding::Mp3 as i32,
            "wav" => AudioEncoding::Linear16 as i32,
            "ogg" => AudioEncoding::OggOpus as i32,
            _ => AudioEncoding::Unspecified as i32,
        };

        Self {
            audio_encoding: encoding,
            speaking_rate: value.speaking_rate,
            pitch: value.pitch,
            volume_gain_db: value.volume_gain_db,
            sample_rate_hertz: value.sample_rate_hertz,
            effects_profile_id: value.effects_profile_id,
        }
    }
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

impl From<TtsRequest> for SynthesizeSpeechRequest {
    fn from(value: TtsRequest) -> Self {
        let input = value.text.map_or_else(
            || None,
            |text| {
                Some(SynthesisInput {
                    input_source: Some(InputSource::Text(text)),
                })
            },
        );

        let voice = value
            .voice
            .map_or_else(|| None, |settings| Some(settings.into()));

        let audio_config: Option<AudioConfig> = value
            .audio
            .map_or_else(|| None, |settings| Some(settings.into()));

        Self {
            input,
            voice,
            audio_config,
        }
    }
}
