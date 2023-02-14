use crate::api::requests::CompletionRequest;
use crate::api::responses::CompletionResponse;
use actix_web::{post, web, HttpResponse, Responder};
use async_openai::types::CreateCompletionRequestArgs;
use async_openai::Client;

pub mod requests;
pub mod responses;

#[post("/completion")]
pub async fn completion(
    req: web::Json<CompletionRequest>,
    client: web::Data<Client>,
) -> impl Responder {
    let CompletionRequest {
        prompt,
        max_tokens,
        temperature,
    } = req.into_inner();

    let mut request_args_builder = CreateCompletionRequestArgs::default();
    request_args_builder
        .model("text-davinci-003")
        .prompt(prompt);

    if let Some(tokens) = max_tokens {
        request_args_builder.max_tokens(tokens);
    }

    if let Some(temp) = temperature {
        request_args_builder.temperature(temp);
    }

    let resp = client
        .into_inner()
        .completions()
        .create(request_args_builder.build().unwrap())
        .await
        .unwrap();

    resp.choices
        .iter()
        .for_each(|c| println!("Choice: {:?}", c));

    HttpResponse::Ok().json(CompletionResponse {
        data: resp.choices.first().unwrap().text.clone(),
    })
}
