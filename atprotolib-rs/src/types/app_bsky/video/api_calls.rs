use crate::{
    api_calls::{AddApiAuth, ApiAuthConfig, ApiError},
    types::app_bsky
};

pub async fn get_job_status(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    job_id: &str
) -> Result<app_bsky::video::GetJobStatusResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.video.getJobStatus", host_name);

    let query_params = vec![("jobId", job_id)];

    let client = reqwest::Client::new();

    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: app_bsky::video::GetJobStatusResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn get_upload_limits(
    host_name: &str,
    api_auth_config: &ApiAuthConfig
) -> Result<app_bsky::video::GetUploadLimitsResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.video.getUploadLimits", host_name);

    let client = reqwest::Client::new();

    let response = client
        .get(&api_url)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: app_bsky::video::GetUploadLimitsResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn upload_video(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    video: Vec<u8>
) -> Result<app_bsky::video::UploadVideoResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.video.uploadVideo", host_name);

    let client = reqwest::Client::new();

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .body(video)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: app_bsky::video::UploadVideoResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}
