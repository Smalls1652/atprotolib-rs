use serde::{Deserialize, Serialize};

/*
    com.atproto.server.resetPassword
*/

/// Represents a password reset request.
///
/// [`com.atproto.server.resetPassword#request`](https://docs.bsky.app/docs/api/com-atproto-server-reset-password#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct ResetPasswordRequest {
    #[serde(rename = "token")]
    pub token: String,

    #[serde(rename = "password")]
    pub password: String
}
