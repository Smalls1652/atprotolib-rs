use crate::{
    api_calls::{AddApiAuth, ApiAuthConfig, ApiError},
    types::app_bsky
};

pub async fn get_actor_feeds(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    actor: &str,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<app_bsky::feed::GetActorFeedsResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.feed.getActorFeeds", host_name);

    let mut query_params = Vec::new();
    query_params.push(("actor", actor));

    let limit_string = limit.unwrap_or_else(|| 50).to_string();
    query_params.push(("limit", limit_string.as_str()));

    if let Some(cursor) = cursor {
        query_params.push(("cursor", cursor));
    }

    let client = reqwest::Client::new();

    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: app_bsky::feed::GetActorFeedsResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn get_actor_likes(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    actor: &str,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<app_bsky::feed::GetActorLikesResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.feed.getActorLikes", host_name);

    let mut query_params = Vec::new();
    query_params.push(("actor", actor));

    let limit_string = limit.unwrap_or_else(|| 50).to_string();
    query_params.push(("limit", limit_string.as_str()));

    if let Some(cursor) = cursor {
        query_params.push(("cursor", cursor));
    }

    let client = reqwest::Client::new();

    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: app_bsky::feed::GetActorLikesResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn get_author_feed(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    actor: &str,
    limit: Option<i32>,
    cursor: Option<&str>,
    filter: Option<&str>,
    include_pins: bool
) -> Result<app_bsky::feed::GetAuthorFeedResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.feed.getAuthorFeed", host_name);

    let mut query_params = Vec::new();
    query_params.push(("actor", actor));

    let limit_string = limit.unwrap_or_else(|| 50).to_string();
    query_params.push(("limit", limit_string.as_str()));

    if let Some(cursor) = cursor {
        query_params.push(("cursor", cursor));
    }

    if let Some(filter) = filter {
        query_params.push(("filter", filter));
    }

    let include_pins_string = include_pins.to_string();
    query_params.push(("includePins", include_pins_string.as_str()));

    let client = reqwest::Client::new();

    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: app_bsky::feed::GetAuthorFeedResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}
