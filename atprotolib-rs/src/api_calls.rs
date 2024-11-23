use reqwest::{RequestBuilder, Response, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct ApiAuthConfig {
    pub data: ApiAuthConfigData
}

#[derive(Clone)]
pub enum ApiAuthConfigData {
    None,
    AdminUser(ApiAuthAdminUser),
    BearerToken(ApiAuthBearerToken)
}

#[derive(Clone)]
pub struct ApiAuthAdminUser {
    pub username: String,
    pub password: String
}

#[derive(Clone)]
pub struct ApiAuthBearerToken {
    pub token: String
}

pub trait AddApiAuth {
    fn add_api_auth(
        self,
        api_auth_config: ApiAuthConfig
    ) -> Self;
}

impl AddApiAuth for RequestBuilder {
    fn add_api_auth(
        self,
        api_auth_config: ApiAuthConfig
    ) -> Self {
        match api_auth_config.data {
            ApiAuthConfigData::AdminUser(auth) => {
                self.basic_auth(auth.username, Some(auth.password))
            }

            ApiAuthConfigData::BearerToken(auth) => self.bearer_auth(auth.token),

            ApiAuthConfigData::None => self
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(skip, default)]
    pub kind: ApiErrorKind
}

impl ApiError {
    pub async fn new(response: Response) -> Result<Self, Box<dyn std::error::Error>> {
        let status = response.status();
        let text = response.text().await?;
        let error: ApiError = serde_json::from_str(&text)?;

        let kind = match status {
            StatusCode::BAD_REQUEST => ApiErrorKind::BadRequest,
            StatusCode::UNAUTHORIZED => ApiErrorKind::Unauthorized,
            StatusCode::FORBIDDEN => ApiErrorKind::Forbidden,
            StatusCode::NOT_FOUND => ApiErrorKind::NotFound,
            StatusCode::METHOD_NOT_ALLOWED => ApiErrorKind::MethodNotAllowed,
            StatusCode::CONFLICT => ApiErrorKind::Conflict,
            StatusCode::INTERNAL_SERVER_ERROR => ApiErrorKind::InternalServerError,
            StatusCode::SERVICE_UNAVAILABLE => ApiErrorKind::ServiceUnavailable,
            StatusCode::GATEWAY_TIMEOUT => ApiErrorKind::GatewayTimeout,
            _ => ApiErrorKind::Unknown
        };

        Ok(ApiError {
            error: error.error,
            message: error.message,
            kind
        })
    }
}

impl std::error::Error for ApiError {}

impl std::fmt::Display for ApiError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter
    ) -> std::fmt::Result {
        write!(f, "API Error: {} - {}", self.error, self.message)
    }
}

#[derive(Debug, Clone)]
pub enum ApiErrorKind {
    Unknown,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    Conflict,
    InternalServerError,
    ServiceUnavailable,
    GatewayTimeout
}

impl Default for ApiErrorKind {
    fn default() -> Self {
        ApiErrorKind::Unknown
    }
}

impl std::fmt::Display for ApiErrorKind {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter
    ) -> std::fmt::Result {
        match self {
            ApiErrorKind::Unknown => write!(f, "Unknown"),
            ApiErrorKind::BadRequest => write!(f, "Bad Request"),
            ApiErrorKind::Unauthorized => write!(f, "Unauthorized"),
            ApiErrorKind::Forbidden => write!(f, "Forbidden"),
            ApiErrorKind::NotFound => write!(f, "Not Found"),
            ApiErrorKind::MethodNotAllowed => write!(f, "Method Not Allowed"),
            ApiErrorKind::Conflict => write!(f, "Conflict"),
            ApiErrorKind::InternalServerError => write!(f, "Internal Server Error"),
            ApiErrorKind::ServiceUnavailable => write!(f, "Service Unavailable"),
            ApiErrorKind::GatewayTimeout => write!(f, "Gateway Timeout")
        }
    }
}
