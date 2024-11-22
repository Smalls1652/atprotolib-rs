use serde::{Deserialize, Serialize};

/*
    com.atproto.server.createSession
*/

/// Represents a session creation request.
///
/// [`com.atproto.server.createSession#request`](https://docs.bsky.app/docs/api/com-atproto-server-create-session#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSessionRequest {
    /// Handle or other identifier supported by the server for the
    /// authenticating user.
    #[serde(rename = "identifier")]
    pub identifier: String,

    #[serde(rename = "password")]
    pub password: String,

    #[serde(rename = "authFactorToken", skip_serializing_if = "Option::is_none")]
    pub auth_factor_token: Option<String>
}

/// Represents a session creation response.
///
/// [`com.atproto.server.createSession#responses`](https://docs.bsky.app/docs/api/com-atproto-server-create-session#responses)
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSessionResponse {
    #[serde(rename = "accessJwt")]
    pub access_jwt: String,

    #[serde(rename = "refreshJwt")]
    pub refresh_jwt: String,

    #[serde(rename = "handle")]
    pub handle: String,

    #[serde(rename = "did")]
    pub did: String,

    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "emailConfirmed", default)]
    pub email_confirmed: bool,

    #[serde(rename = "emailAuthFactor", default)]
    pub email_auth_factor: bool,

    #[serde(rename = "active", default)]
    pub active: bool,

    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>
}
