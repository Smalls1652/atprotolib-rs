use crate::{
    api_calls::{AddApiAuth, ApiAuthConfig, ApiError},
    types::app_bsky
};

use super::ProfileView;

pub async fn get_profile(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    actor: &str
) -> Result<app_bsky::actor::ProfileViewDetailed, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.actor.getProfile", host_name);

    let query_params = vec![
        ("actor", actor)
    ];

    let client = reqwest::Client::new();

    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: app_bsky::actor::ProfileViewDetailed = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn get_suggestions(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<app_bsky::actor::GetSuggestionsResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.actor.getSuggestions", host_name);

    let mut query_params = vec![];

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
            let response_body: app_bsky::actor::GetSuggestionsResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn search_actors_typeahead(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    query: &str,
    limit: Option<i32>
) -> Result<app_bsky::actor::SearchActorsTypeaheadResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.actor.searchActorsTypeahead", host_name);

    let mut query_params = Vec::new();
    query_params.push(("q", query));

    let limit_string = limit.unwrap_or_else(|| 10).to_string();
    query_params.push(("limit", limit_string.as_str()));

    let client = reqwest::Client::new();

    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: app_bsky::actor::SearchActorsTypeaheadResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn search_actors(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    query: &str,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<app_bsky::actor::SearchActorsResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.actor.searchActors", host_name);

    let mut query_params = Vec::new();
    query_params.push(("q", query));

    let limit_string = limit.unwrap_or_else(|| 25).to_string();
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
            let response_body: app_bsky::actor::SearchActorsResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}
