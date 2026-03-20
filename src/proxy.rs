use crate::error::ApiError;
use actix_web::HttpResponse;
use log::error;
use log::info;
use reqwest::Client;
use serde_json::Value;

const PAYLOAD_SIZE: u64 = 2 * 1024 * 1024;

pub async fn proxy(client: &Client, endpoint: &str) -> Result<HttpResponse, ApiError> {
    info!("proxying: {}", endpoint);
    let res = client.get(endpoint).send().await.map_err(|e| {
        error!("proxy error: {}", e);
        ApiError::ProxyError
    })?;
    if let Some(payload_size) = res.content_length() {
        if payload_size > PAYLOAD_SIZE {
            error!("proxy error: payload too large");
            return Err(ApiError::ProxyError);
        }
    }
    let json = res.json::<Value>().await.map_err(|e| {
        error!("proxy error: {}", e);
        ApiError::ProxyError
    })?;
    Ok(HttpResponse::Ok().json(json))
}
