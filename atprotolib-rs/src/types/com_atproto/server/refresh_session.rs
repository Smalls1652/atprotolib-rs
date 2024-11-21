use serde::{Deserialize, Serialize};

/*
    com.atproto.server.refreshSession
*/

/// Represents a session refresh response.
///
/// [`com.atproto.server.refreshSession#responses`](https://docs.bsky.app/docs/api/com-atproto-server-refresh-session#responses)
#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshSessionRequest {
    #[serde(rename = "accessJwt")]
    pub access_jwt: String,

    #[serde(rename = "refreshJwt")]
    pub refresh_jwt: String,

    #[serde(rename = "handle")]
    pub handle: String,

    #[serde(rename = "did")]
    pub did: String,

    #[serde(rename = "active")]
    pub active: bool,

    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>
}
