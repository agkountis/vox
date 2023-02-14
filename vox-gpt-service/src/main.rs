use crate::api::completion;
use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer, Scope};

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let bind_address = std::env::var("VOX_GPT_SERVICE_ADDRESS")
        .expect("VOX_GPT_SERVICE_ADDRESS environment variable not set");

    let client = web::Data::new(async_openai::Client::new());

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::permissive())
            .app_data(client.clone())
            .service(Scope::new("api/v1").service(completion))
    })
    .bind(bind_address)?
    .run()
    .await
}
