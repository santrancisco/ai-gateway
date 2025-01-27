use crate::executor::embeddings::handle_embeddings_invoke;
use crate::types::credentials::Credentials;
use actix_web::{web, HttpResponse};
use actix_web::{HttpMessage, HttpRequest};

use crate::types::gateway::{
    CreateEmbeddingRequest, CreateEmbeddingResponse, EmbeddingData, EmbeddingUsage,
};

use crate::handler::AvailableModels;
use crate::handler::CallbackHandlerFn;
use crate::GatewayApiError;

use super::find_model_by_full_name;

pub async fn embeddings_handler(
    request: web::Json<CreateEmbeddingRequest>,
    models: web::Data<AvailableModels>,
    callback_handler: web::Data<CallbackHandlerFn>,
    req: HttpRequest,
) -> Result<HttpResponse, GatewayApiError> {
    let request = request.into_inner();
    let available_models = models.into_inner();
    let llm_model = find_model_by_full_name(&request.model, &available_models)?;
    let key_credentials = req.extensions().get::<Credentials>().cloned();

    let result = handle_embeddings_invoke(
        request,
        callback_handler.get_ref(),
        &llm_model,
        key_credentials.as_ref(),
    )
    .await?;

    let data = result
        .data
        .iter()
        .map(|v| EmbeddingData {
            object: v.object.clone(),
            embedding: v.embedding.clone(),
            index: v.index,
        })
        .collect();

    Ok(HttpResponse::Ok().json(CreateEmbeddingResponse {
        object: "list".into(),
        data,
        model: llm_model.model.clone(),
        usage: EmbeddingUsage {
            prompt_tokens: result.usage.prompt_tokens,
            total_tokens: result.usage.total_tokens,
        },
    }))
}
