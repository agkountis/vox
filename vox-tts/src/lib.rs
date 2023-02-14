use google_cognitive_apis::errors::Result;
use google_cognitive_apis::texttospeech::synthesizer::Synthesizer;
use std::fs;
use std::path::Path;

pub use google_cognitive_apis::api::grpc::google::cloud::texttospeech::v1::synthesis_input::InputSource;
pub use google_cognitive_apis::api::grpc::google::cloud::texttospeech::v1::{
    AudioConfig, AudioEncoding, ListVoicesRequest, ListVoicesResponse, SsmlVoiceGender,
    SynthesisInput, SynthesizeSpeechRequest, SynthesizeSpeechResponse, VoiceSelectionParams,
};

#[derive(Debug, Clone)]
pub struct VoxTts {
    synthesizer: Synthesizer,
}

impl VoxTts {
    pub async fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let credentials = fs::read_to_string(path).unwrap();
        let synthesizer = Synthesizer::create(credentials).await?;

        Ok(Self { synthesizer })
    }

    pub async fn list_voices(&mut self) -> Result<ListVoicesResponse> {
        self.synthesizer
            .list_voices(ListVoicesRequest {
                language_code: "el".to_string(),
            })
            .await
    }

    pub async fn synthesize_speech(
        &mut self,
        request: SynthesizeSpeechRequest,
    ) -> Result<SynthesizeSpeechResponse> {
        self.synthesizer.synthesize_speech(request).await
    }
}
