use serde::{Deserialize, Serialize};

/*
    com.atproto.sync.getLatestCommit
*/

/*    Type: response
    Id: com.atproto.sync.getLatestCommit#response
    Kind: object

    Properties:
    - cid: string (JsonProperty: cid) [Required]
    - rev: string (JsonProperty: rev) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetLatestCommitResponse {
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "rev")]
    pub rev: String
}
