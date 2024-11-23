use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::com_atproto::{admin::RepoRef, repo::StrongRef};

/*
    com.atproto.moderation.createReport
*/

/*    Type: request
    Id: com.atproto.moderation.createReport#request
    Kind: object

    Properties:
    - reason_type: com.atproto.moderation.defs#reasonType (JsonProperty: reasonType) [Required]
    - reason: string (JsonProperty: reason) [Optional]
    - subject: union  (JsonProperty: subject) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateReportRequest {
    #[serde(rename = "reasonType")]
    pub reason_type: String,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "subject")]
    pub subject: CreateReportRequestSubject
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CreateReportRequestSubject {
    RepoRef(RepoRef),
    StrongRef(StrongRef)
}

/*    Type: response
    Id: com.atproto.moderation.createReport#response
    Kind: object

    Properties:
    - id: integer  (JsonProperty: id) [Required]
    - reason_type: com.atproto.moderation.defs#reasonType (JsonProperty: reasonType) [Required]
    - reason: string (JsonProperty: reason) [Optional]
    - subject: union  (JsonProperty: subject) [Required]
    - reported_by: string (JsonProperty: reportedBy) [Required]
    - created_at: datetime (JsonProperty: createdAt) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateReportResponse {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "reasonType")]
    pub reason_type: String,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "subject")]
    pub subject: serde_json::Value,
    #[serde(rename = "reportedBy")]
    pub reported_by: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>
}
