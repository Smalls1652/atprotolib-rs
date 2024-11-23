use serde::{Deserialize, Serialize};

/*
    com.atproto.sync.listBlobs
*/

/*    Type: response
    Id: com.atproto.sync.listBlobs#response
    Kind: object

    Properties:
    - cursor: string (JsonProperty: cursor) [Optional]
    - cids: string[] (JsonProperty: cids) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct ListBlobsResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "cids")]
    pub cids: Vec<String>
}
