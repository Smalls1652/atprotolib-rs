use serde::{Deserialize, Serialize};

/*
    com.atproto.admin.updateAccountEmail
*/

/// Represents a request to update an account's email.
/// 
/// [`com.atproto.admin.updateAccountEmail#request`](https://docs.bsky.app/docs/api/com-atproto-admin-update-account-email#request)
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountEmailRequest {
    #[serde(rename = "account")]
    pub account: String,

    #[serde(rename = "email")]
    pub email: String
}
