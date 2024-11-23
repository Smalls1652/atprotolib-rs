use serde::{Deserialize, Serialize};

use super::Label;

/*
    com.atproto.label.queryLabels
*/

/*    Type: response
    Id: com.atproto.label.queryLabels#response
    Kind: object

    Properties:
    - cursor: string (JsonProperty: cursor) [Optional]
    - labels: com.atproto.label.defs#label[] (JsonProperty: labels) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct QueryLabelsResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "labels")]
    pub labels: Vec<Label>
}
