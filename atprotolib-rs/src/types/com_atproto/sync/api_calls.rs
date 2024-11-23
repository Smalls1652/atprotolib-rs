use crate::{
    api_calls::{AddApiAuth, ApiAuthConfig, ApiError},
    types::com_atproto::sync::{ListReposResponse, GetLatestCommitResponse, GetRepoStatusResponse, ListBlobsResponse, NotifyOfUpdateRequest, RequestCrawlRequest}
};

pub async fn get_blob(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    did: &str,
    cid: &str
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.sync.getBlob", host_name);

    let query_params = vec![
        ("did", did),
        ("cid", cid)
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
            let response_body = response.bytes().await?;
            Ok(response_body.to_vec())
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn get_latest_commit(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    did: &str
) -> Result<GetLatestCommitResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.sync.getLatestCommit", host_name);

    let query_params = vec![
        ("did", did)
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
            let response_body: GetLatestCommitResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn get_record(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    did: &str,
    collection: &str,
    rkey: &str
) -> Result<String, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.sync.getRecord", host_name);

    let query_params = vec![
        ("did", did),
        ("collection", collection),
        ("rkey", rkey)
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
            let response_body = response.text().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn get_repo_status(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    did: &str
) -> Result<GetRepoStatusResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.sync.getRepoStatus", host_name);

    let query_params = vec![
        ("did", did)
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
            let response_body: GetRepoStatusResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn get_repo(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    did: &str,
    since: Option<&str>
) -> Result<String, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.sync.getRepo", host_name);

    let mut query_params = Vec::new();
    query_params.push(("did", did));

    if let Some(since) = since {
        query_params.push(("since", since));
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
            let response_body = response.text().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn list_blobs(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    did: String,
    since: Option<String>,
    limit: Option<i32>,
    cursor: Option<String>
) -> Result<ListBlobsResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.sync.listBlobs", host_name);

    let mut query_params = Vec::new();
    query_params.push(("did", did));
    query_params.push(("limit", limit.unwrap_or_else(|| 500).to_string()));

    if let Some(since) = since {
        query_params.push(("since", since));
    }

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
            let response_body: ListBlobsResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn list_repos(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    limit: Option<i32>,
    cursor: Option<String>
) -> Result<ListReposResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.sync.listRepos", host_name);

    let mut query_params = Vec::new();
    query_params.push(("limit", limit.unwrap_or_else(|| 100).to_string()));

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
            let response_body: ListReposResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn notify_of_update(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: NotifyOfUpdateRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.sync.notifyOfUpdate", host_name);

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

pub async fn request_crawl(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: RequestCrawlRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.sync.requestCrawl", host_name);

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
