use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{app_bsky::{actor::{ProfileView, ProfileViewBasic}, embed::{ExternalEmbedView, ImageEmbedView, RecordEmbedView, RecordWithMediaEmbedView, VideoEmbedView}, graph::ListViewBasic}, com_atproto::label::Label};

/*    Type: postView
    Id: app.bsky.feed.defs#postView
    Kind: object
    
    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - cid: string (JsonProperty: cid) [Required]
    - author: app.bsky.actor.defs#profileViewBasic (JsonProperty: author) [Required]
    - record: unknown  (JsonProperty: record) [Required]
    - embed: union  (JsonProperty: embed) [Optional]
    - reply_count: integer  (JsonProperty: replyCount) [Optional]
    - repost_count: integer  (JsonProperty: repostCount) [Optional]
    - like_count: integer  (JsonProperty: likeCount) [Optional]
    - quote_count: integer  (JsonProperty: quoteCount) [Optional]
    - indexed_at: datetime (JsonProperty: indexedAt) [Required]
    - viewer: #viewerState (JsonProperty: viewer) [Optional]
    - labels: com.atproto.label.defs#label[] (JsonProperty: labels) [Optional]
    - threadgate: #threadgateView (JsonProperty: threadgate) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#postView")]
pub struct PostView {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "author")]
    pub author: ProfileViewBasic,
    #[serde(rename = "record")]
    pub record: serde_json::Value,
    #[serde(rename = "embed", skip_serializing_if = "Option::is_none")]
    pub embed: Option<PostViewEmbed>,
    #[serde(rename = "replyCount", default)]
    pub reply_count: i32,
    #[serde(rename = "repostCount", default)]
    pub repost_count: i32,
    #[serde(rename = "likeCount", default)]
    pub like_count: i32,
    #[serde(rename = "quoteCount", default)]
    pub quote_count: i32,
    #[serde(rename = "indexedAt")]
    pub indexed_at: DateTime<Utc>,
    #[serde(rename = "viewer", skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerState>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
    #[serde(rename = "threadgate", skip_serializing_if = "Option::is_none")]
    pub threadgate: Option<ThreadgateView>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostViewEmbed {
    Images(ImageEmbedView),
    Video(VideoEmbedView),
    External(ExternalEmbedView),
    Record(RecordEmbedView),
    RecordWithMedia(RecordWithMediaEmbedView)
}

/*    Type: viewerState
    Id: app.bsky.feed.defs#viewerState
    Kind: object
    
    Properties:
    - repost: string (JsonProperty: repost) [Optional]
    - like: string (JsonProperty: like) [Optional]
    - thread_muted: boolean  (JsonProperty: threadMuted) [Optional]
    - reply_disabled: boolean  (JsonProperty: replyDisabled) [Optional]
    - embedding_disabled: boolean  (JsonProperty: embeddingDisabled) [Optional]
    - pinned: boolean  (JsonProperty: pinned) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#viewerState")]
pub struct ViewerState {
    #[serde(rename = "repost", skip_serializing_if = "Option::is_none")]
    pub repost: Option<String>,
    #[serde(rename = "like", skip_serializing_if = "Option::is_none")]
    pub like: Option<String>,
    #[serde(rename = "threadMuted", default)]
    pub thread_muted: bool,
    #[serde(rename = "replyDisabled", default)]
    pub reply_disabled: bool,
    #[serde(rename = "embeddingDisabled", default)]
    pub embedding_disabled: bool,
    #[serde(rename = "pinned", default)]
    pub pinned: bool
}

/*    Type: feedViewPost
    Id: app.bsky.feed.defs#feedViewPost
    Kind: object
    
    Properties:
    - post: #postView (JsonProperty: post) [Required]
    - reply: #replyRef (JsonProperty: reply) [Optional]
    - reason: union  (JsonProperty: reason) [Optional]
    - feed_context: string (JsonProperty: feedContext) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#feedViewPost")]
pub struct FeedViewPost {
    #[serde(rename = "post")]
    pub post: PostView,
    #[serde(rename = "reply", skip_serializing_if = "Option::is_none")]
    pub reply: Option<ReplyRef>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<FeedViewPostReason>,
    #[serde(rename = "feedContext", skip_serializing_if = "Option::is_none")]
    pub feed_context: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FeedViewPostReason {
    Repost(ReasonRepost),
    Pin(ReasonPin)
}

/*    Type: replyRef
    Id: app.bsky.feed.defs#replyRef
    Kind: object
    
    Properties:
    - root: union  (JsonProperty: root) [Required]
    - parent: union  (JsonProperty: parent) [Required]
    - grandparent_author: app.bsky.actor.defs#profileViewBasic (JsonProperty: grandparentAuthor) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#replyRef")]
pub struct ReplyRef {
    #[serde(rename = "root")]
    pub root: ReplyRefItem,
    #[serde(rename = "parent")]
    pub parent: ReplyRefItem,
    #[serde(rename = "grandparentAuthor", skip_serializing_if = "Option::is_none")]
    pub grandparent_author: Option<ProfileViewBasic>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ReplyRefItem {
    Post(PostView),
    NotFoundPost(NotFoundPost),
    BlockedPost(BlockedPost)
}

/*    Type: reasonRepost
    Id: app.bsky.feed.defs#reasonRepost
    Kind: object
    
    Properties:
    - by: app.bsky.actor.defs#profileViewBasic (JsonProperty: by) [Required]
    - indexed_at: datetime (JsonProperty: indexedAt) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#reasonRepost")]
pub struct ReasonRepost {
    #[serde(rename = "by")]
    pub by: ProfileViewBasic,
    #[serde(rename = "indexedAt")]
    pub indexed_at: DateTime<Utc>
}

/*    Type: reasonPin
    Id: app.bsky.feed.defs#reasonPin
    Kind: object
    
    Properties:
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#reasonPin")]
pub struct ReasonPin {}

/*    Type: threadViewPost
    Id: app.bsky.feed.defs#threadViewPost
    Kind: object
    
    Properties:
    - post: #postView (JsonProperty: post) [Required]
    - parent: union  (JsonProperty: parent) [Optional]
    - replies: union[] (JsonProperty: replies) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#threadViewPost")]
pub struct ThreadViewPost {
    #[serde(rename = "post")]
    pub post: PostView,
    #[serde(rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<ThreadViewPostItem>,
    #[serde(rename = "replies", skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<ThreadViewPostItem>>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ThreadViewPostItem {
    ThreadPost(Box<ThreadViewPost>),
    NotFoundPost(NotFoundPost),
    BlockedPost(BlockedPost)
}

/*    Type: notFoundPost
    Id: app.bsky.feed.defs#notFoundPost
    Kind: object
    
    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - not_found: boolean  (JsonProperty: notFound) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#notFoundPost")]
pub struct NotFoundPost {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "notFound", default)]
    pub not_found: bool
}

/*    Type: blockedPost
    Id: app.bsky.feed.defs#blockedPost
    Kind: object
    
    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - blocked: boolean  (JsonProperty: blocked) [Required]
    - author: #blockedAuthor (JsonProperty: author) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#blockedPost")]
pub struct BlockedPost {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "blocked", default)]
    pub blocked: bool,
    #[serde(rename = "author")]
    pub author: BlockedAuthor
}

/*    Type: blockedAuthor
    Id: app.bsky.feed.defs#blockedAuthor
    Kind: object
    
    Properties:
    - did: string (JsonProperty: did) [Required]
    - viewer: app.bsky.actor.defs#viewerState (JsonProperty: viewer) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#blockedAuthor")]
pub struct BlockedAuthor {
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "viewer", skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerState>
}

/*    Type: generatorView
    Id: app.bsky.feed.defs#generatorView
    Kind: object
    
    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - cid: string (JsonProperty: cid) [Required]
    - did: string (JsonProperty: did) [Required]
    - creator: app.bsky.actor.defs#profileView (JsonProperty: creator) [Required]
    - display_name: string (JsonProperty: displayName) [Required]
    - description: string (JsonProperty: description) [Optional]
    - description_facets: app.bsky.richtext.facet[] (JsonProperty: descriptionFacets) [Optional]
    - avatar: string (JsonProperty: avatar) [Optional]
    - like_count: integer  (JsonProperty: likeCount) [Optional]
    - accepts_interactions: boolean  (JsonProperty: acceptsInteractions) [Optional]
    - labels: com.atproto.label.defs#label[] (JsonProperty: labels) [Optional]
    - viewer: #generatorViewerState (JsonProperty: viewer) [Optional]
    - indexed_at: datetime (JsonProperty: indexedAt) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#generatorView")]
pub struct GeneratorView {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "creator")]
    pub creator: ProfileView,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "descriptionFacets", skip_serializing_if = "Option::is_none")]
    pub description_facets: Option<Vec<serde_json::Value>>,
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(rename = "likeCount", default)]
    pub like_count: i32,
    #[serde(rename = "acceptsInteractions", default)]
    pub accepts_interactions: bool,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
    #[serde(rename = "viewer", skip_serializing_if = "Option::is_none")]
    pub viewer: Option<GeneratorViewerState>,
    #[serde(rename = "indexedAt")]
    pub indexed_at: DateTime<Utc>
}

/*    Type: generatorViewerState
    Id: app.bsky.feed.defs#generatorViewerState
    Kind: object
    
    Properties:
    - like: string (JsonProperty: like) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#generatorViewerState")]
pub struct GeneratorViewerState {
    #[serde(rename = "like", skip_serializing_if = "Option::is_none")]
    pub like: Option<String>
}

/*    Type: skeletonFeedPost
    Id: app.bsky.feed.defs#skeletonFeedPost
    Kind: object
    
    Properties:
    - post: string (JsonProperty: post) [Required]
    - reason: union  (JsonProperty: reason) [Optional]
    - feed_context: string (JsonProperty: feedContext) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#skeletonFeedPost")]
pub struct SkeletonFeedPost {
    #[serde(rename = "post")]
    pub post: String,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<SkeletonFeedPostReason>,
    #[serde(rename = "feedContext", skip_serializing_if = "Option::is_none")]
    pub feed_context: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SkeletonFeedPostReason {
    SkeletonReasonRepost(SkeletonReasonRepost),
    SkeletonReasonPin(SkeletonReasonPin)
}

/*    Type: skeletonReasonRepost
    Id: app.bsky.feed.defs#skeletonReasonRepost
    Kind: object
    
    Properties:
    - repost: string (JsonProperty: repost) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#skeletonReasonRepost")]
pub struct SkeletonReasonRepost {
    #[serde(rename = "repost")]
    pub repost: String
}

/*    Type: skeletonReasonPin
    Id: app.bsky.feed.defs#skeletonReasonPin
    Kind: object
    
    Properties:
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#skeletonReasonPin")]
pub struct SkeletonReasonPin {}

/*    Type: threadgateView
    Id: app.bsky.feed.defs#threadgateView
    Kind: object
    
    Properties:
    - uri: string (JsonProperty: uri) [Optional]
    - cid: string (JsonProperty: cid) [Optional]
    - record: unknown  (JsonProperty: record) [Optional]
    - lists: app.bsky.graph.defs#listViewBasic[] (JsonProperty: lists) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#threadgateView")]
pub struct ThreadgateView {
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(rename = "record", skip_serializing_if = "Option::is_none")]
    pub record: Option<serde_json::Value>,
    #[serde(rename = "lists", skip_serializing_if = "Option::is_none")]
    pub lists: Option<Vec<ListViewBasic>>
}

/*    Type: interaction
    Id: app.bsky.feed.defs#interaction
    Kind: object
    
    Properties:
    - item: string (JsonProperty: item) [Optional]
    - event: string (JsonProperty: event) [Optional]
    - feed_context: string (JsonProperty: feedContext) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#interaction")]
pub struct Interaction {
    #[serde(rename = "item", skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(rename = "feedContext", skip_serializing_if = "Option::is_none")]
    pub feed_context: Option<String>
}
