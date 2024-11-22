use serde::{Deserialize, Serialize};

/*
    com.atproto.sync.listRepos
*/

/*    Type: response
    Id: com.atproto.sync.listRepos#response
    Kind: object
    
    Properties:
    - cursor: string (JsonProperty: cursor) [Optional]
    - repos: #repo[] (JsonProperty: repos) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct ListReposResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "repos")]
    pub repos: Vec<Repo>
}

/*    Type: repo
    Id: com.atproto.sync.listRepos#repo
    Kind: object
    
    Properties:
    - did: string (JsonProperty: did) [Required]
    - head: string (JsonProperty: head) [Required]
    - rev: string (JsonProperty: rev) [Required]
    - active: boolean  (JsonProperty: active) [Optional]
    - status: string (JsonProperty: status) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "head")]
    pub head: String,
    #[serde(rename = "rev")]
    pub rev: String,
    #[serde(rename = "active", default)]
    pub active: bool,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>
}
