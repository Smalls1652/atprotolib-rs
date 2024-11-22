use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/*
    com.atproto.sync.subscribeRepos
*/

/*    Type: commit
    Id: com.atproto.sync.subscribeRepos#commit
    Kind: object
    
    Properties:
    - seq: integer  (JsonProperty: seq) [Required]
    - rebase: boolean  (JsonProperty: rebase) [Required]
    - too_big: boolean  (JsonProperty: tooBig) [Required]
    - repo: string (JsonProperty: repo) [Required]
    - commit: cid-link  (JsonProperty: commit) [Required]
    - prev: cid-link  (JsonProperty: prev) [Optional]
    - rev: string (JsonProperty: rev) [Required]
    - since: string (JsonProperty: since) [Required]
    - blocks: bytes  (JsonProperty: blocks) [Required]
    - ops: #repoOp[] (JsonProperty: ops) [Required]
    - blobs: cid-link[] (JsonProperty: blobs) [Required]
    - time: datetime (JsonProperty: time) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    #[serde(rename = "seq", default)]
    pub seq: i32,
    #[serde(rename = "rebase", default)]
    pub rebase: bool,
    #[serde(rename = "tooBig", default)]
    pub too_big: bool,
    #[serde(rename = "repo")]
    pub repo: String,
    #[serde(rename = "commit")]
    pub commit: String,
    #[serde(rename = "prev", skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(rename = "rev")]
    pub rev: String,
    #[serde(rename = "since")]
    pub since: String,
    #[serde(rename = "blocks")]
    pub blocks: Vec<u8>,
    #[serde(rename = "ops")]
    pub ops: Vec<RepoOp>,
    #[serde(rename = "blobs")]
    pub blobs: Vec<String>,
    #[serde(rename = "time")]
    pub time: String
}

/*    Type: identity
    Id: com.atproto.sync.subscribeRepos#identity
    Kind: object
    
    Properties:
    - seq: integer  (JsonProperty: seq) [Required]
    - did: string (JsonProperty: did) [Required]
    - time: datetime (JsonProperty: time) [Required]
    - handle: string (JsonProperty: handle) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Identity {
    #[serde(rename = "seq", default)]
    pub seq: i32,
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "time")]
    pub time: DateTime<Utc>,
    #[serde(rename = "handle", skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>
}

/*    Type: account
    Id: com.atproto.sync.subscribeRepos#account
    Kind: object
    
    Properties:
    - seq: integer  (JsonProperty: seq) [Required]
    - did: string (JsonProperty: did) [Required]
    - time: datetime (JsonProperty: time) [Required]
    - active: boolean  (JsonProperty: active) [Required]
    - status: string (JsonProperty: status) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    #[serde(rename = "seq", default)]
    pub seq: i32,
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "time")]
    pub time: DateTime<Utc>,
    #[serde(rename = "active", default)]
    pub active: bool,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>
}

/*    Type: handle
    Id: com.atproto.sync.subscribeRepos#handle
    Kind: object
    
    Properties:
    - seq: integer  (JsonProperty: seq) [Required]
    - did: string (JsonProperty: did) [Required]
    - handle: string (JsonProperty: handle) [Required]
    - time: datetime (JsonProperty: time) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Handle {
    #[serde(rename = "seq", default)]
    pub seq: i32,
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "handle")]
    pub handle: String,
    #[serde(rename = "time")]
    pub time: DateTime<Utc>
}

/*    Type: migrate
    Id: com.atproto.sync.subscribeRepos#migrate
    Kind: object
    
    Properties:
    - seq: integer  (JsonProperty: seq) [Required]
    - did: string (JsonProperty: did) [Required]
    - migrate_to: string (JsonProperty: migrateTo) [Required]
    - time: datetime (JsonProperty: time) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Migrate {
    #[serde(rename = "seq", default)]
    pub seq: i32,
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "migrateTo")]
    pub migrate_to: String,
    #[serde(rename = "time")]
    pub time: DateTime<Utc>
}

/*    Type: tombstone
    Id: com.atproto.sync.subscribeRepos#tombstone
    Kind: object
    
    Properties:
    - seq: integer  (JsonProperty: seq) [Required]
    - did: string (JsonProperty: did) [Required]
    - time: datetime (JsonProperty: time) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Tombstone {
    #[serde(rename = "seq", default)]
    pub seq: i32,
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "time")]
    pub time: DateTime<Utc>
}

/*    Type: info
    Id: com.atproto.sync.subscribeRepos#info
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

/*    Type: repoOp
    Id: com.atproto.sync.subscribeRepos#repoOp
    Kind: object
    
    Properties:
    - action: string (JsonProperty: action) [Required]
    - path: string (JsonProperty: path) [Required]
    - cid: cid-link  (JsonProperty: cid) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RepoOp {
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "cid")]
    pub cid: String
}


