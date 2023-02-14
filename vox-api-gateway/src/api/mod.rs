use crate::api::requests::{CompletionRequest, InteractionRequest, TtsRequest};
use crate::api::responses::{CompletionResponse, InteractionResponse, TtsResponse};
use crate::ServiceUris;
use actix_web::{post, web, HttpResponse, Responder};

pub mod requests;
pub mod responses;

#[post("/interact")]
pub async fn interact(
    req: web::Json<InteractionRequest>,
    data: web::Data<ServiceUris>,
) -> impl Responder {
    let InteractionRequest {
        prompt,
        speech_settings,
    } = req.into_inner();

    println!("{:?}", speech_settings);

    let http_client = reqwest::Client::new();
    let CompletionResponse {
        data: completion_data,
    } = http_client
        .post(data.gpt_service_url.as_str())
        .json(&CompletionRequest {
            prompt,
            max_tokens: Some(2048u16),
            temperature: Some(0.5),
        })
        .send()
        .await
        .unwrap()
        .json::<CompletionResponse>()
        .await
        .unwrap();

    let resp = {
        if let Some(speech) = speech_settings {
            let tts_request = TtsRequest {
                text: Some(completion_data.clone()),
                voice: speech.voice,
                audio: speech.audio,
            };

            http_client
                .post(data.tts_service_url.as_str())
                .json(&tts_request)
                .send()
                .await
                .unwrap()
                .json::<TtsResponse>()
                .await
                .ok()
        } else {
            None
        }
    }
    .map_or_else(
        || InteractionResponse {
            text: completion_data.clone(),
            audio: None,
        },
        |resp| InteractionResponse {
            text: completion_data.clone(),
            audio: Some(resp.audio),
        },
    );

    HttpResponse::Ok().json(resp)
}

// #[post("/tts")]
// pub async fn tts() -> impl Responder {
//     todo!()
// }
//
// #[post("/stt")]
// pub async fn stt() -> impl Responder {
//     todo!()
// }
