use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::Cursor;
use std::{fs, io};
use vox_tts::{
    AudioConfig, AudioEncoding, InputSource, SsmlVoiceGender, SynthesisInput,
    SynthesizeSpeechRequest, VoiceSelectionParams, VoxTts,
};

use kira::manager::MainPlaybackState;
use kira::sound::static_sound::PlaybackState;
use kira::{
    manager::{backend::cpal::CpalBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The language of the narrator
    #[arg(short, long, value_name = "LANGUAGE")]
    language: Option<String>,

    /// The gender of the narrator
    #[arg(short, long, value_name = "GENDER")]
    gender: Option<String>,

    /// The text to narrate
    text: String,
}

fn format_language(language: &str) -> String {
    if let Some((a, b)) = language.split_once('-') {
        return format!("{}-{}", a.to_lowercase(), b.to_uppercase());
    }

    language.to_string()
}

fn derive_gender_code(gender: &str) -> i32 {
    let g = gender.to_lowercase();
    match g.as_str() {
        "male" => 1,
        "female" => 2,
        _ => 0, // Unspecified
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let text = cli.text;

    let mut tts = VoxTts::new("data/psyched-age-376411-4596362e14f5.json")
        .await
        .unwrap();

    let language_code = if let Some(lang) = cli.language {
        format_language(&lang)
    } else {
        "en-GB".to_string()
    };

    let voice_name = format!("{}-Wavenet-A", &language_code);

    let gender = if let Some(g) = cli.gender {
        derive_gender_code(&g)
    } else {
        SsmlVoiceGender::Female as i32
    };

    let response = tts
        .synthesize_speech(SynthesizeSpeechRequest {
            input: Some(SynthesisInput {
                input_source: Some(InputSource::Text(text)),
            }),
            voice: Some(VoiceSelectionParams {
                language_code,
                name: voice_name,
                ssml_gender: gender,
            }),
            audio_config: Some(AudioConfig {
                audio_encoding: AudioEncoding::Mp3 as i32,
                speaking_rate: 1f64,
                pitch: 0f64,
                volume_gain_db: 0f64,
                sample_rate_hertz: 24000,
                effects_profile_id: vec![],
            }),
        })
        .await
        .unwrap();

    // Create an audio manager. This plays sounds and manages resources.
    let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap();
    let sound_data = StaticSoundData::from_cursor(
        Cursor::new(response.audio_content),
        StaticSoundSettings::default(),
    )
    .unwrap();
    let handle = manager.play(sound_data).unwrap();

    loop {
        if handle.state() != PlaybackState::Playing {
            return;
        }
    }
}
