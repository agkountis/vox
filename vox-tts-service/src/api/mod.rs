use crate::api::requests::TtsRequest;
use crate::api::responses::TtsResponse;
use actix_web::{post, web, HttpResponse};
use base64::Engine;
use vox_tts::VoxTts;

pub mod requests;
pub mod responses;

#[post("/tts")]
pub async fn tts(req: web::Json<TtsRequest>) -> HttpResponse {
    let mut vox_tts = VoxTts::new("data/psyched-age-376411-4596362e14f5.json")
        .await
        .unwrap();

    let resp = vox_tts
        .synthesize_speech(req.into_inner().into())
        .await
        .unwrap();
    HttpResponse::Ok().json(TtsResponse {
        audio: base64::engine::general_purpose::STANDARD_NO_PAD.encode(resp.audio_content),
    })
}
