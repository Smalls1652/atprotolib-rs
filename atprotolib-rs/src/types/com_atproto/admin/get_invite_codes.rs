use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/*
    com.atproto.admin.getInviteCodes
*/

/// Represents a response to get invite codes.
/// 
/// [`com.atproto.admin.getInviteCodes#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-get-invite-codes#responses)
#[derive(Debug, Serialize, Deserialize)]
pub struct GetInviteCodesResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    #[serde(rename = "codes")]
    pub codes: Vec<GetInviteCodesResponseCode>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetInviteCodesResponseCode {
    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "available")]
    pub available: i32,

    #[serde(rename = "disabled")]
    pub disabled: bool,

    #[serde(rename = "forAccount")]
    pub for_account: String,

    #[serde(rename = "createdBy")]
    pub created_by: String,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    #[serde(rename = "uses")]
    pub uses: Vec<GetInviteCodesResponseCodeUse>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetInviteCodesResponseCodeUse {
    #[serde(rename = "usedBy")]
    pub used_by: String,

    #[serde(rename = "usedAt")]
    pub used_at: DateTime<Utc>
}
