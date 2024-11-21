use serde::{Deserialize, Serialize};

use super::CommitMeta;

/*
    com.atproto.repo.applyWrites
*/

/*    Type: request
    Id: com.atproto.repo.applyWrites#request
    Kind: object

    Properties:
    - repo: string (JsonProperty: repo) [Required]
    - validate: boolean  (JsonProperty: validate) [Optional]
    - writes: union[] (JsonProperty: writes) [Required]
    - swap_commit: string (JsonProperty: swapCommit) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct ApplyWritesRequest {
    #[serde(rename = "repo")]
    pub repo: String,
    #[serde(rename = "validate")]
    pub validate: bool,
    #[serde(rename = "writes")]
    pub writes: Vec<ApplyWritesRequestWrites>,
    #[serde(rename = "swapCommit", skip_serializing_if = "Option::is_none")]
    pub swap_commit: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ApplyWritesRequestWrites {
    Create(Create),
    Update(Update),
    Delete(Delete)
}

/*    Type: response
    Id: com.atproto.repo.applyWrites#response
    Kind: object

    Properties:
    - commit: com.atproto.repo.defs#commitMeta (JsonProperty: commit) [Optional]
    - results: union[] (JsonProperty: results) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct ApplyWritesResponse {
    #[serde(rename = "commit", skip_serializing_if = "Option::is_none")]
    pub commit: Option<CommitMeta>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ApplyWritesResponseResults>>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ApplyWritesResponseResults {
    CreateResult(CreateResult),
    UpdateResult(UpdateResult),
    DeleteResult(DeleteResult)
}

/*    Type: create
    Id: com.atproto.repo.applyWrites#create
    Kind: object

    Properties:
    - collection: string (JsonProperty: collection) [Required]
    - rkey: string (JsonProperty: rkey) [Optional]
    - value: unknown  (JsonProperty: value) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.repo.applyWrites#create")]
pub struct Create {
    #[serde(rename = "collection")]
    pub collection: String,
    #[serde(rename = "rkey", skip_serializing_if = "Option::is_none")]
    pub rkey: Option<String>,
    #[serde(rename = "value")]
    pub value: serde_json::Value
}

/*    Type: update
    Id: com.atproto.repo.applyWrites#update
    Kind: object

    Properties:
    - collection: string (JsonProperty: collection) [Required]
    - rkey: string (JsonProperty: rkey) [Required]
    - value: unknown  (JsonProperty: value) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.repo.applyWrites#update")]
pub struct Update {
    #[serde(rename = "collection")]
    pub collection: String,
    #[serde(rename = "rkey")]
    pub rkey: String,
    #[serde(rename = "value")]
    pub value: serde_json::Value
}

/*    Type: delete
    Id: com.atproto.repo.applyWrites#delete
    Kind: object

    Properties:
    - collection: string (JsonProperty: collection) [Required]
    - rkey: string (JsonProperty: rkey) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.repo.applyWrites#delete")]
pub struct Delete {
    #[serde(rename = "collection")]
    pub collection: String,
    #[serde(rename = "rkey")]
    pub rkey: String
}

/*    Type: createResult
    Id: com.atproto.repo.applyWrites#createResult
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - cid: string (JsonProperty: cid) [Required]
    - validation_status: string (JsonProperty: validationStatus) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.repo.applyWrites#createResult")]
pub struct CreateResult {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "validationStatus", skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>
}

/*    Type: updateResult
    Id: com.atproto.repo.applyWrites#updateResult
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - cid: string (JsonProperty: cid) [Required]
    - validation_status: string (JsonProperty: validationStatus) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.repo.applyWrites#updateResult")]
pub struct UpdateResult {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "validationStatus", skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>
}

/*    Type: deleteResult
    Id: com.atproto.repo.applyWrites#deleteResult
    Kind: object

    Properties:
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.repo.applyWrites#deleteResult")]
pub struct DeleteResult {}
