use crate::{
    api_calls::{AddApiAuth, ApiAuthConfig, ApiError},
    types::com_atproto::server::{
        ConfirmEmailRequest,
        CreateAccountRequest,
        CreateAccountResponse,
        CreateAppPasswordRequest,
        CreateAppPasswordResponse,
        CreateSessionRequest,
        CreateSessionResponse,
        DeactivateAccountRequest,
        DeleteAccountRequest,
        DescribeServerResponse,
        GetServiceAuthResponse,
        GetSessionResponse,
        InviteCodeRequest,
        InviteCodeResponse,
        ListAppPasswordsResponse,
        RefreshSessionResponse,
        RequestEmailUpdateResponse,
        RequestPasswordResetRequest,
        ReserveSigningKeyRequest,
        ReserveSigningKeyResponse,
        ResetPasswordRequest,
        RevokeAppPasswordRequest,
        UpdateEmailRequest
    }
};

pub async fn confirm_email(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: ConfirmEmailRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.server.confirmEmail", host_name);

    let client = reqwest::Client::new();

    let request_builder = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&request);

    let response = request_builder.send().await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn create_account(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: CreateAccountRequest
) -> Result<CreateAccountResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.createAccount",
        host_name
    );

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
            let response_body: CreateAccountResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn create_app_password(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: CreateAppPasswordRequest
) -> Result<CreateAppPasswordResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.createAppPassword",
        host_name
    );

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
            let response_body: CreateAppPasswordResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn create_invite_code(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: InviteCodeRequest
) -> Result<InviteCodeResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.createInviteCode",
        host_name
    );

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
            let response_body: InviteCodeResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn create_session(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: CreateSessionRequest
) -> Result<CreateSessionResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.createSession",
        host_name
    );

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
            let response_body: CreateSessionResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn deactivate_account(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: DeactivateAccountRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.deactivateAccount",
        host_name
    );

    let client = reqwest::Client::new();

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn delete_account(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: DeleteAccountRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.deleteAccount",
        host_name
    );

    let client = reqwest::Client::new();

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn describe_server(
    host_name: &str,
    api_auth_config: &ApiAuthConfig
) -> Result<DescribeServerResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.describeServer",
        host_name
    );

    let client = reqwest::Client::new();

    let response = client
        .get(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: DescribeServerResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn get_service_auth(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    did: String,
    expiry: i32,
    lexicon: Option<String>
) -> Result<GetServiceAuthResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.getServiceAuth",
        host_name
    );

    let mut query_params = Vec::new();
    query_params.push(("aud", did));
    query_params.push(("exp", expiry.to_string()));

    if let Some(lexicon) = lexicon {
        query_params.push(("lexicon", lexicon));
    }

    let client = reqwest::Client::new();

    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .send()
        .await?;

    match response.error_for_status() {
        Ok(response) => {
            let response_body: GetServiceAuthResponse = response.json().await?;
            Ok(response_body)
        }
        Err(err) => Err(Box::new(err))
    }
}

pub async fn get_session(
    host_name: &str,
    api_auth_config: &ApiAuthConfig
) -> Result<GetSessionResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.server.getSession", host_name);

    let client = reqwest::Client::new();

    let response = client
        .get(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: GetSessionResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn list_app_passwords(
    host_name: &str,
    api_auth_config: &ApiAuthConfig
) -> Result<ListAppPasswordsResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.listAppPasswords",
        host_name
    );

    let client = reqwest::Client::new();

    let response = client
        .get(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: ListAppPasswordsResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn refresh_session(
    host_name: &str,
    api_auth_config: &ApiAuthConfig
) -> Result<RefreshSessionResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.refreshSession",
        host_name
    );

    let client = reqwest::Client::new();

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: RefreshSessionResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn request_email_update(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    email: String
) -> Result<RequestEmailUpdateResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.requestEmailUpdate",
        host_name
    );

    let client = reqwest::Client::new();

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&email)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: RequestEmailUpdateResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn request_password_reset(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: RequestPasswordResetRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.requestPasswordReset",
        host_name
    );

    let client = reqwest::Client::new();

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn reserve_signing_key(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: ReserveSigningKeyRequest
) -> Result<ReserveSigningKeyResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.reserveSigningKey",
        host_name
    );

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
            let response_body: ReserveSigningKeyResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn reset_password(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: ResetPasswordRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.resetPassword",
        host_name
    );

    let client = reqwest::Client::new();

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn revoke_app_password(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: RevokeAppPasswordRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.revokeAppPassword",
        host_name
    );

    let client = reqwest::Client::new();

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

pub async fn update_email(
    host_name: &str,
    api_auth_config: &ApiAuthConfig,
    request: UpdateEmailRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.server.updateEmail", host_name);

    let client = reqwest::Client::new();

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}
