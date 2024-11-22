use serde::{Deserialize, Serialize};

/*
    app.bsky.feed.post
*/

/*    Type: replyRef
    Id: app.bsky.feed.post#replyRef
    Kind: object
    
    Properties:
    - root: com.atproto.repo.strongRef (JsonProperty: root) [Required]
    - parent: com.atproto.repo.strongRef (JsonProperty: parent) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct PostReplyRef {
    #[serde(rename = "root")]
    pub root: serde_json::Value,
    #[serde(rename = "parent")]
    pub parent: serde_json::Value,
}

/*    Type: entity
    Id: app.bsky.feed.post#entity
    Kind: object
    
    Properties:
    - index: #textSlice (JsonProperty: index) [Required]
    - type: string (JsonProperty: type) [Required]
    - value: string (JsonProperty: value) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct PostEntity {
    #[serde(rename = "index")]
    pub index: PostTextSlice,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "value")]
    pub value: String,
}

/*    Type: textSlice
    Id: app.bsky.feed.post#textSlice
    Kind: object
    
    Properties:
    - start: integer  (JsonProperty: start) [Required]
    - end: integer  (JsonProperty: end) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct PostTextSlice {
    #[serde(rename = "start", default)]
    pub start: i32,
    #[serde(rename = "end", default)]
    pub end: i32,
}
