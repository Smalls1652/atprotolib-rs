use serde::{Deserialize, Serialize};

/*
    com.atproto.server.getSession
*/

/// Represents a session request.
///
/// [`com.atproto.server.getSession#request`](https://docs.bsky.app/docs/api/com-atproto-server-get-session#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct GetSessionResponse {
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

    #[serde(rename = "emailConfirmed")]
    pub email_confirmed: bool,

    #[serde(rename = "emailAuthFactor")]
    pub email_auth_factor: bool,

    #[serde(rename = "active")]
    pub active: bool,

    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>
}
