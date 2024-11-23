use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{app_bsky::{embed::{ExternalEmbed, ImageEmbed}, richtext::RichTextFacet}, com_atproto::{label::SelfLabels, repo::{Record, StrongRef}}};

/*
    app.bsky.feed.post
*/

/*    Type: post
    Id: app.bsky.feed.post
    Kind: object
    
    Properties:
    - text: string (JsonProperty: text) [Required]
    - facets: #app.bsky.richtext.facet[] (JsonProperty: facets) [Optional]
    - reply_ref: app.bsky.feed.post#replyRef (JsonProperty: replyRef) [Optional]
    - embed: union (JsonProperty: embed) [Optional]
    - langs: string[] (JsonProperty: langs) [Optional]
    - labels: union (JsonProperty: labels) [Optional]
    - tags: string[] (JsonProperty: tags) [Optional]
    - created_at: datetime (JsonProperty: createdAt) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.post")]
pub struct Post {
    #[serde(rename = "$type")]
    pub type_: String,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "facets", skip_serializing_if = "Option::is_none")]
    pub facets: Option<Vec<RichTextFacet>>,
    #[serde(rename = "replyRef", skip_serializing_if = "Option::is_none")]
    pub reply_ref: Option<PostReplyRef>,
    #[serde(rename = "embed", skip_serializing_if = "Option::is_none")]
    pub embed: Option<PostEmbeds>,
    #[serde(rename = "langs", skip_serializing_if = "Option::is_none")]
    pub langs: Option<Vec<String>>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<PostLabels>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

impl Post {
    pub fn new(
        text: String,
        created_at: DateTime<Utc>,
        langs : Option<Vec<String>>,
    ) -> Post {;
        Post {
            type_: "app.bsky.feed.post".to_string(),
            text,
            facets: None,
            reply_ref: None,
            embed: None,
            langs: Some(langs.unwrap_or_else(|| vec!["en".to_string()])),
            labels: None,
            tags: None,
            created_at
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostEmbeds {
    Images(ImageEmbed),
    External(ExternalEmbed)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostLabels {
    SelfLabels(SelfLabels)
}

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
    pub root: StrongRef,
    #[serde(rename = "parent")]
    pub parent: StrongRef,
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
