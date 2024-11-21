use serde::{Deserialize, Serialize};

/*    Type: commitMeta
    Id: com.atproto.repo.defs#commitMeta
    Kind: object
    
    Properties:
    - cid: string (JsonProperty: cid) [Required]
    - rev: string (JsonProperty: rev) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.repo.defs#commitMeta")]
pub struct CommitMeta {
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "rev")]
    pub rev: String
}

/*
    Type: strongRef
    Id: com.atproto.repo.strongRef
    Kind: object

    Properties:
    - did: string (JsonProperty: did) [Required]
    - cid: string (JsonProperty: cid) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.repo.strongRef")]
pub struct StrongRef {
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "cid")]
    pub cid: String
}
