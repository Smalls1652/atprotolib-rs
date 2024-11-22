use serde::{Deserialize, Serialize};

/*
    com.atproto.server.checkAccountStatus
*/

/// Represents an account status response.
///
/// [`com.atproto.server.checkAccountStatus#responses`](https://docs.bsky.app/docs/api/com-atproto-server-check-account-status#responses)
#[derive(Serialize, Deserialize, Debug)]
pub struct CheckAccountStatusResponse {
    /// Whether the account is activated.
    #[serde(rename = "activated", default)]
    pub activated: bool,

    /// Whether the account's DID is valid.
    #[serde(rename = "validDid", default)]
    pub valid_did: bool,

    #[serde(rename = "repoCommit")]
    pub repo_commit: String,

    #[serde(rename = "repoRev")]
    pub repo_revision: String,

    #[serde(rename = "repoBlocks")]
    pub repo_blocks: String,

    #[serde(rename = "indexedRecords")]
    pub indexed_records: i32,

    #[serde(rename = "privateStateValues")]
    pub private_state_values: i32,

    #[serde(rename = "publicStateValues")]
    pub expected_blobs: i32,

    #[serde(rename = "importedBlobs")]
    pub imported_blobs: i32
}
