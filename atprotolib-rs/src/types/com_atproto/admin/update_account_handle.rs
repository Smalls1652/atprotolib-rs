use serde::{Deserialize, Serialize};

/*
    com.atproto.admin.updateAccountHandle
*/

/// Represents a request to update an account's handle.
/// 
/// [`com.atproto.admin.updateAccountHandle#request`](https://docs.bsky.app/docs/api/com-atproto-admin-update-account-handle#request)
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountHandleRequest {
    #[serde(rename = "did")]
    pub did: String,

    #[serde(rename = "handle")]
    pub handle: String
}
