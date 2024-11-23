use crate::{
    api_calls::{AddApiAuth, ApiAuthConfig, ApiError},
    types::com_atproto
};

pub async fn get_recommended_did_credentials(
    host_name: &str,
    api_auth_config: &ApiAuthConfig
) -> Result<com_atproto::identity::GetRecommendedDidCredentialsResponse, Box<dyn std::error::Error>>
{
    let api_url = format!(
        "https://{}/xrpc/com.atproto.identity.getRecommendedDidCredentials",
        host_name
    );

    let client = reqwest::Client::new();

    let response = client
        .get(&api_url)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: com_atproto::identity::GetRecommendedDidCredentialsResponse =
                response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn resolve_handle(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    handle: &str
) -> Result<com_atproto::identity::ResolveHandleResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.identity.resolveHandle",
        host_name
    );

    let query_params = vec![("handle", handle)];

    let client = reqwest::Client::new();

    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: com_atproto::identity::ResolveHandleResponse =
                response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn sign_plc_operation(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: com_atproto::identity::SignPlcOperationRequest
) -> Result<com_atproto::identity::SignPlcOperationResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.identity.signPlcOperation",
        host_name
    );

    let client = reqwest::Client::new();

    let response = client
        .post(&api_url)
        .json(&request)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: com_atproto::identity::SignPlcOperationResponse =
                response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn submit_plc_operation(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: com_atproto::identity::SubmitPlcOperationRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.identity.submitPlcOperation",
        host_name
    );

    let client = reqwest::Client::new();

    let response = client
        .post(&api_url)
        .json(&request)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn update_handle(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: com_atproto::identity::UpdateHandleRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.identity.updateHandle",
        host_name
    );

    let client = reqwest::Client::new();

    let response = client
        .post(&api_url)
        .json(&request)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}
