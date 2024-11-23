use serde::{Deserialize, Serialize};

use super::Label;

/*
    com.atproto.label.subscribeLabels
*/

/*    Type: labels
    Id: com.atproto.label.subscribeLabels#labels
    Kind: object

    Properties:
    - seq: integer  (JsonProperty: seq) [Required]
    - labels: com.atproto.label.defs#label[] (JsonProperty: labels) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Labels {
    #[serde(rename = "seq")]
    pub seq: i64,
    #[serde(rename = "labels")]
    pub labels: Vec<Label>
}

/*    Type: info
    Id: com.atproto.label.subscribeLabels#info
    Kind: object

    Properties:
    - name: string (JsonProperty: name) [Required]
    - message: string (JsonProperty: message) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>
}
