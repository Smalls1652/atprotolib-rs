use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::com_atproto::server::InviteCode;

/*    Type: statusAttr
    Id: com.atproto.admin.defs#statusAttr
    Kind: object

    Properties:
    - applied: boolean  (JsonProperty: applied) [Required]
    - ref: string (JsonProperty: ref) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.admin.defs#statusAttr")]
pub struct StatusAttr {
    #[serde(rename = "applied", default)]
    pub applied: bool,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>
}

/*    Type: accountView
    Id: com.atproto.admin.defs#accountView
    Kind: object

    Properties:
    - did: string (JsonProperty: did) [Required]
    - handle: string (JsonProperty: handle) [Required]
    - email: string (JsonProperty: email) [Optional]
    - related_records: unknown[] (JsonProperty: relatedRecords) [Optional]
    - indexed_at: datetime (JsonProperty: indexedAt) [Required]
    - invited_by: com.atproto.server.defs#inviteCode (JsonProperty: invitedBy) [Optional]
    - invites: com.atproto.server.defs#inviteCode[] (JsonProperty: invites) [Optional]
    - invites_disabled: boolean  (JsonProperty: invitesDisabled) [Optional]
    - email_confirmed_at: datetime (JsonProperty: emailConfirmedAt) [Optional]
    - invite_note: string (JsonProperty: inviteNote) [Optional]
    - deactivated_at: datetime (JsonProperty: deactivatedAt) [Optional]
    - threat_signatures: #threatSignature[] (JsonProperty: threatSignatures) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.admin.defs#accountView")]
pub struct AccountView {
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "handle")]
    pub handle: String,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "relatedRecords", skip_serializing_if = "Option::is_none")]
    pub related_records: Option<Vec<serde_json::Value>>,
    #[serde(rename = "indexedAt")]
    pub indexed_at: DateTime<Utc>,
    #[serde(rename = "invitedBy", skip_serializing_if = "Option::is_none")]
    pub invited_by: Option<InviteCode>,
    #[serde(rename = "invites", skip_serializing_if = "Option::is_none")]
    pub invites: Option<Vec<InviteCode>>,
    #[serde(rename = "invitesDisabled", default)]
    pub invites_disabled: bool,
    #[serde(rename = "emailConfirmedAt", skip_serializing_if = "Option::is_none")]
    pub email_confirmed_at: Option<DateTime<Utc>>,
    #[serde(rename = "inviteNote", skip_serializing_if = "Option::is_none")]
    pub invite_note: Option<String>,
    #[serde(rename = "deactivatedAt", skip_serializing_if = "Option::is_none")]
    pub deactivated_at: Option<DateTime<Utc>>,
    #[serde(rename = "threatSignatures", skip_serializing_if = "Option::is_none")]
    pub threat_signatures: Option<Vec<ThreatSignature>>
}

/*    Type: repoRef
    Id: com.atproto.admin.defs#repoRef
    Kind: object

    Properties:
    - did: string (JsonProperty: did) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.admin.defs#repoRef")]
pub struct RepoRef {
    #[serde(rename = "did")]
    pub did: String
}

/*    Type: repoBlobRef
    Id: com.atproto.admin.defs#repoBlobRef
    Kind: object

    Properties:
    - did: string (JsonProperty: did) [Required]
    - cid: string (JsonProperty: cid) [Required]
    - record_uri: string (JsonProperty: recordUri) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.admin.defs#repoBlobRef")]
pub struct RepoBlobRef {
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "recordUri", skip_serializing_if = "Option::is_none")]
    pub record_uri: Option<String>
}

/*    Type: threatSignature
    Id: com.atproto.admin.defs#threatSignature
    Kind: object

    Properties:
    - property: string (JsonProperty: property) [Required]
    - value: string (JsonProperty: value) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.admin.defs#threatSignature")]
pub struct ThreatSignature {
    #[serde(rename = "property")]
    pub property: String,
    #[serde(rename = "value")]
    pub value: String
}
