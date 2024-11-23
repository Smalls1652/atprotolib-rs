use serde::{Deserialize, Serialize};

/*
    com.atproto.admin.sendEmail
*/

/// Represents a request to send an email.
///
/// [`com.atproto.admin.sendEmail#request`](https://docs.bsky.app/docs/api/com-atproto-admin-send-email#request)
#[derive(Debug, Serialize, Deserialize)]
pub struct SendEmailRequest {
    #[serde(rename = "recipientDid")]
    pub recipient_did: String,

    #[serde(rename = "content")]
    pub content: String,

    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    #[serde(rename = "senderDid")]
    pub sender_did: String,

    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>
}

/// Represents a response to a request to send an email.
///
/// [`com.atproto.admin.sendEmail#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-send-email#responses)
#[derive(Debug, Serialize, Deserialize)]
pub struct SendEmailResponse {
    #[serde(rename = "sent", default)]
    pub sent: bool
}
