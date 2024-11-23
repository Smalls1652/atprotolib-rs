use crate::{
    api_calls::{AddApiAuth, ApiAuthConfig, ApiError},
    types::com_atproto
};

pub async fn create_report(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    report: com_atproto::moderation::CreateReportRequest
) -> Result<com_atproto::moderation::CreateReportResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.moderation.createReport", host_name);

    let client = reqwest::Client::new();

    let response = client
        .post(&api_url)
        .json(&report)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: com_atproto::moderation::CreateReportResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}
