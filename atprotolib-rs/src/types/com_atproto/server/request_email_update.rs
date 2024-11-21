use serde::{Deserialize, Serialize};

/*
    com.atproto.server.requestEmailUpdate
*/

/// Represents an email update response.
///
/// [`com.atproto.server.requestEmailUpdate#responses`](https://docs.bsky.app/docs/api/com-atproto-server-request-email-update#responses)
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestEmailUpdateResponse {
    #[serde(rename = "tokenRequired")]
    pub token_required: bool
}
