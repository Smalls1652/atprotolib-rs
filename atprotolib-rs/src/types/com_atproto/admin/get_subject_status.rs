use serde::{Deserialize, Serialize};

/*
    com.atproto.admin.getSubjectStatus
*/

/// Represents a response to get the status of an Account subject.
///
/// [`com.atproto.admin.getSubjectStatus#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-get-subject-status#responses)
#[derive(Debug, Serialize, Deserialize)]
pub struct GetSubjectStatusAccountResponse {
    #[serde(rename = "did")]
    pub did: String,

    #[serde(rename = "takedown")]
    pub takedown: GetSubjectStatusResponseStatus,

    #[serde(rename = "deactivated")]
    pub deactivated: GetSubjectStatusResponseStatus
}

/// Represents a response to get the status of a Record subject.
///
/// [`com.atproto.admin.getSubjectStatus#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-get-subject-status#responses)
#[derive(Debug, Serialize, Deserialize)]
pub struct GetSubjectStatusRecordResponse {
    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "cid")]
    pub cid: String,

    #[serde(rename = "takedown")]
    pub takedown: GetSubjectStatusResponseStatus,

    #[serde(rename = "deactivated")]
    pub deactivated: GetSubjectStatusResponseStatus
}

/// Represents a response to get the status of a Blob subject.
///
/// [`com.atproto.admin.getSubjectStatus#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-get-subject-status#responses)
#[derive(Debug, Serialize, Deserialize)]
pub struct GetSubjectStatusBlobResponse {
    #[serde(rename = "did")]
    pub did: String,

    #[serde(rename = "cid")]
    pub cid: String,

    #[serde(rename = "recordUri", skip_serializing_if = "Option::is_none")]
    pub record_uri: Option<String>,

    #[serde(rename = "takedown")]
    pub takedown: GetSubjectStatusResponseStatus,

    #[serde(rename = "deactivated")]
    pub deactivated: GetSubjectStatusResponseStatus
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSubjectStatusResponseStatus {
    #[serde(rename = "applied", default)]
    pub applied: bool,

    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>
}
