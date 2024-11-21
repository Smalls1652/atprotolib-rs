use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{app_bsky::actor::ProfileView, com_atproto::label::Label};

/*
    app.bsky.notification.listNotifications
*/

/*    Type: response
    Id: app.bsky.notification.listNotifications#response
    Kind: object

    Properties:
    - cursor: string (JsonProperty: cursor) [Optional]
    - notifications: #notification[] (JsonProperty: notifications) [Required]
    - priority: boolean  (JsonProperty: priority) [Optional]
    - seen_at: datetime (JsonProperty: seenAt) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct ListNotificationsResponse {
    cursor: Option<String>,
    notifications: Vec<Notification>,
    priority: bool,
    seen_at: Option<DateTime<Utc>>
}

/*    Type: notification
    Id: app.bsky.notification.listNotifications#notification
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - cid: string (JsonProperty: cid) [Required]
    - author: app.bsky.actor.defs#profileView (JsonProperty: author) [Required]
    - reason: string (JsonProperty: reason) [Required]
    - reason_subject: string (JsonProperty: reasonSubject) [Optional]
    - record: unknown  (JsonProperty: record) [Required]
    - is_read: boolean  (JsonProperty: isRead) [Required]
    - indexed_at: datetime (JsonProperty: indexedAt) [Required]
    - labels: com.atproto.label.defs#label[] (JsonProperty: labels) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "author")]
    pub author: ProfileView,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "reasonSubject", skip_serializing_if = "Option::is_none")]
    pub reason_subject: Option<String>,
    #[serde(rename = "record")]
    pub record: serde_json::Value,
    #[serde(rename = "isRead")]
    pub is_read: bool,
    #[serde(rename = "indexedAt")]
    pub indexed_at: DateTime<Utc>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>
}
