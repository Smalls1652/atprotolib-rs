use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{
    app_bsky::actor::ProfileView,
    com_atproto::label::{Label, LabelValueDefinition}
};

/*    Type: labelerView
    Id: app.bsky.labeler.defs#labelerView
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - cid: string (JsonProperty: cid) [Required]
    - creator: app.bsky.actor.defs#profileView (JsonProperty: creator) [Required]
    - like_count: integer  (JsonProperty: likeCount) [Optional]
    - viewer: #labelerViewerState (JsonProperty: viewer) [Optional]
    - indexed_at: datetime (JsonProperty: indexedAt) [Required]
    - labels: com.atproto.label.defs#label[] (JsonProperty: labels) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.labeler.defs#labelerView")]
pub struct LabelerView {
    uri: String,
    cid: String,
    creator: ProfileView,
    like_count: i32,
    viewer: Option<LabelerViewerState>,
    indexed_at: DateTime<Utc>,
    labels: Option<Vec<Label>>
}

/*    Type: labelerViewDetailed
    Id: app.bsky.labeler.defs#labelerViewDetailed
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - cid: string (JsonProperty: cid) [Required]
    - creator: app.bsky.actor.defs#profileView (JsonProperty: creator) [Required]
    - policies: app.bsky.labeler.defs#labelerPolicies (JsonProperty: policies) [Required]
    - like_count: integer  (JsonProperty: likeCount) [Optional]
    - viewer: #labelerViewerState (JsonProperty: viewer) [Optional]
    - indexed_at: datetime (JsonProperty: indexedAt) [Required]
    - labels: com.atproto.label.defs#label[] (JsonProperty: labels) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct LabelerViewDetailed {
    uri: String,
    cid: String,
    creator: ProfileView,
    policies: LabelerPolicies,
    like_count: i32,
    viewer: Option<LabelerViewerState>,
    indexed_at: DateTime<Utc>,
    labels: Option<Vec<Label>>
}

/*    Type: labelerViewerState
    Id: app.bsky.labeler.defs#labelerViewerState
    Kind: object

    Properties:
    - like: string (JsonProperty: like) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct LabelerViewerState {
    like: Option<String>
}

/*    Type: labelerPolicies
    Id: app.bsky.labeler.defs#labelerPolicies
    Kind: object

    Properties:
    - label_values: com.atproto.label.defs#labelValue[] (JsonProperty: labelValues) [Required]
    - label_value_definitions: com.atproto.label.defs#labelValueDefinition[] (JsonProperty: labelValueDefinitions) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct LabelerPolicies {
    label_values: Vec<String>,
    label_value_definitions: Option<Vec<LabelValueDefinition>>
}
