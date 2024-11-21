use serde::{Deserialize, Serialize};

/*
    com.atproto.server.revokeAppPassword
*/

/// Represents an app password revocation request.
///
/// [`com.atproto.server.revokeAppPassword#request`](https://docs.bsky.app/docs/api/com-atproto-server-revoke-app-password#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct RevokeAppPasswordRequest {
    #[serde(rename = "name")]
    pub name: String
}
