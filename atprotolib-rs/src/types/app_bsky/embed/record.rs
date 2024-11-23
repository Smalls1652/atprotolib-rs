use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{
    app_bsky::{
        actor::ProfileViewBasic,
        feed::{BlockedAuthor, GeneratorView},
        graph::{ListView, StarterPackViewBasic},
        labeler::LabelerView
    },
    com_atproto::label::Label
};

/*
    app.bsky.embed.record
*/

/*    Type: view
    Id: app.bsky.embed.record#view
    Kind: object

    Properties:
    - record: union  (JsonProperty: record) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.embed.record#view")]
pub struct RecordEmbedView {
    #[serde(rename = "record")]
    pub record: RecordEmbedUnion
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RecordEmbedUnion {
    RecordEmbedViewRecord(RecordEmbedViewRecord),
    RecordEmbedViewNotFound(RecordEmbedViewNotFound),
    RecordEmbedViewBlocked(RecordEmbedViewBlocked),
    RecordEmbedViewDetached(RecordEmbedViewDetached),
    GeneratorView(GeneratorView),
    ListView(ListView),
    StarterPackViewBasic(StarterPackViewBasic),
    LabelerView(LabelerView)
}

/*    Type: viewRecord
    Id: app.bsky.embed.record#viewRecord
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - cid: string (JsonProperty: cid) [Required]
    - author: app.bsky.actor.defs#profileViewBasic (JsonProperty: author) [Required]
    - value: unknown  (JsonProperty: value) [Required]
    - labels: com.atproto.label.defs#label[] (JsonProperty: labels) [Optional]
    - reply_count: integer  (JsonProperty: replyCount) [Optional]
    - repost_count: integer  (JsonProperty: repostCount) [Optional]
    - like_count: integer  (JsonProperty: likeCount) [Optional]
    - quote_count: integer  (JsonProperty: quoteCount) [Optional]
    - embeds: union[] (JsonProperty: embeds) [Optional]
    - indexed_at: datetime (JsonProperty: indexedAt) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.embed.record#viewRecord")]
pub struct RecordEmbedViewRecord {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "author")]
    pub author: ProfileViewBasic,
    #[serde(rename = "value")]
    pub value: serde_json::Value,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
    #[serde(rename = "replyCount", default)]
    pub reply_count: i32,
    #[serde(rename = "repostCount", default)]
    pub repost_count: i32,
    #[serde(rename = "likeCount", default)]
    pub like_count: i32,
    #[serde(rename = "quoteCount", default)]
    pub quote_count: i32,
    #[serde(rename = "embeds", skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<RecordEmbedUnion>>,
    #[serde(rename = "indexedAt")]
    pub indexed_at: DateTime<Utc>
}

/*    Type: viewNotFound
    Id: app.bsky.embed.record#viewNotFound
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - not_found: boolean  (JsonProperty: notFound) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.embed.record#viewNotFound")]
pub struct RecordEmbedViewNotFound {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "notFound", default)]
    pub not_found: bool
}

/*    Type: viewBlocked
    Id: app.bsky.embed.record#viewBlocked
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - blocked: boolean  (JsonProperty: blocked) [Required]
    - author: app.bsky.feed.defs#blockedAuthor (JsonProperty: author) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.embed.record#viewBlocked")]
pub struct RecordEmbedViewBlocked {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "blocked", default)]
    pub blocked: bool,
    #[serde(rename = "author")]
    pub author: BlockedAuthor
}

/*    Type: viewDetached
    Id: app.bsky.embed.record#viewDetached
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - detached: boolean  (JsonProperty: detached) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.embed.record#viewDetached")]
pub struct RecordEmbedViewDetached {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "detached", default)]
    pub detached: bool
}
