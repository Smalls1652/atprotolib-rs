use serde::{Deserialize, Serialize};

use super::get_account_info::GetAccountInfoResponse;

/*
    com.atproto.admin.searchAccounts
*/

/// Represents a response to a search for accounts.
/// 
/// [`com.atproto.admin.searchAccounts#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-search-accounts#responses)
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAccountsResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    #[serde(rename = "accounts")]
    pub accounts: Vec<GetAccountInfoResponse>
}