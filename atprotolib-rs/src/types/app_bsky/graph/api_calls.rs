use crate::{
    api_calls::{AddApiAuth, ApiAuthConfig, ApiError},
    types::app_bsky
};

pub async fn get_actor_starter_packs(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    actor: &str,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<app_bsky::graph::GetActorStarterPacksResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.getActorStarterPacks", host_name);

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
            let response_body: app_bsky::graph::GetActorStarterPacksResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn get_blocks(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<app_bsky::graph::GetBlocksResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.getBlocks", host_name);

    let mut query_params = Vec::new();

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
            let response_body: app_bsky::graph::GetBlocksResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn get_followers(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    actor: &str,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<app_bsky::graph::GetFollowersResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.getFollowers", host_name);

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
            let response_body: app_bsky::graph::GetFollowersResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn get_follows(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    actor: &str,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<app_bsky::graph::GetFollowsResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.getFollows", host_name);

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
            let response_body: app_bsky::graph::GetFollowsResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn get_known_followers(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    actor: &str,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<app_bsky::graph::GetKnownFollowersResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.getKnownFollowers", host_name);

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
            let response_body: app_bsky::graph::GetKnownFollowersResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn get_list_blocks(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<app_bsky::graph::GetListBlocksResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.getListBlocks", host_name);

    let mut query_params = Vec::new();

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
            let response_body: app_bsky::graph::GetListBlocksResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn get_list_mutes(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<app_bsky::graph::GetListMutesResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.getListMutes", host_name);

    let mut query_params = Vec::new();

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
            let response_body: app_bsky::graph::GetListMutesResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn get_list(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    list: &str,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<app_bsky::graph::GetListResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.getList", host_name);

    let mut query_params = Vec::new();
    query_params.push(("list", list));

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
            let response_body: app_bsky::graph::GetListResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn get_lists(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    actor: &str,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<app_bsky::graph::GetListsResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.getLists", host_name);

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
            let response_body: app_bsky::graph::GetListsResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn get_mutes(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<app_bsky::graph::GetMutesResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.getMutes", host_name);

    let mut query_params = Vec::new();

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
            let response_body: app_bsky::graph::GetMutesResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn get_starter_pack(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    starter_pack: &str
) -> Result<app_bsky::graph::GetStarterPackResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.getStarterPack", host_name);

    let query_params = vec![
        ("starter_pack", starter_pack)
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
            let response_body: app_bsky::graph::GetStarterPackResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn get_suggested_follows_by_actor(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    actor: &str
) -> Result<app_bsky::graph::GetSuggestedFollowsByActorResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.getSuggestedFollowsByActor", host_name);

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
            let response_body: app_bsky::graph::GetSuggestedFollowsByActorResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn mute_actor_list(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: app_bsky::graph::MuteActorListRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.muteActorList", host_name);

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

pub async fn mute_actor(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: app_bsky::graph::MuteActorRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.muteActor", host_name);

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

pub async fn mute_thread(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: app_bsky::graph::MuteThreadRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.muteThread", host_name);

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

pub async fn unmute_actor_list(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: app_bsky::graph::UnmuteActorListRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.unmuteActorList", host_name);

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

pub async fn unmute_actor(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: app_bsky::graph::UnmuteActorRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.unmuteActor", host_name);

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

pub async fn unmute_thread(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: app_bsky::graph::UnmuteThreadRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.unmuteThread", host_name);

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
