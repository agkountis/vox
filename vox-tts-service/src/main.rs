mod api;

use crate::api::tts;
use actix_cors::Cors;
use actix_web::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let bind_address = std::env::var("VOX_TTS_SERVICE_ADDRESS")
        .expect("VOX_TTS_SERVICE_ADDRESS environment variable not set");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::permissive())
            .service(Scope::new("api/v1").service(tts))
    })
    .bind(bind_address)?
    .run()
    .await
}
