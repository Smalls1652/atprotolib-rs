use crate::{
    api_calls::{AddApiAuth, ApiAuthConfig, ApiError},
    types::com_atproto::repo::{ApplyWritesRequest, ApplyWritesResponse}
};

pub async fn apply_writes(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: ApplyWritesRequest
) -> Result<ApplyWritesResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.repo.applyWrites", host_name);

    let client = reqwest::Client::new();

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: ApplyWritesResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}
