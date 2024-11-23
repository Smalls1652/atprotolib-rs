use serde::{Deserialize, Serialize};

/*
    com.atproto.sync.getRepoStatus
*/

/*    Type: response
    Id: com.atproto.sync.getRepoStatus#response
    Kind: object

    Properties:
    - did: string (JsonProperty: did) [Required]
    - active: boolean  (JsonProperty: active) [Required]
    - status: string (JsonProperty: status) [Optional]
    - rev: string (JsonProperty: rev) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetRepoStatusResponse {
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "active", default)]
    pub active: bool,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "rev", skip_serializing_if = "Option::is_none")]
    pub rev: Option<String>
}
