use crate::api::interact;
use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer, Scope};

mod api;

pub struct ServiceUris {
    pub gpt_service_url: String,
    pub tts_service_url: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let bind_address = std::env::var("VOX_API_GATEWAY_ADDRESS")
        .expect("VOX_API_GATEWAY_ADDRESS environment variable not set");

    let gpt_service_address = std::env::var("VOX_GPT_SERVICE_ADDRESS")
        .expect("VOX_GPT_SERVICE_ADDRESS environment variable not set");

    let tts_service_address = std::env::var("VOX_TTS_SERVICE_ADDRESS")
        .expect("VOX_TTS_SERVICE_ADDRESS environment variable not set");

    let data = web::Data::new(ServiceUris {
        gpt_service_url: format!("http://{}/api/v1/completion", gpt_service_address),
        tts_service_url: format!("http://{}/api/v1/tts", tts_service_address),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::permissive())
            .app_data(data.clone())
            .service(Scope::new("api/v1").service(interact))
    })
    .bind(bind_address)?
    .run()
    .await
}
