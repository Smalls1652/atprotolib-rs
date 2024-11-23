use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{
    app_bsky::{
        actor::{ProfileView, ProfileViewBasic},
        feed::GeneratorView,
        richtext::RichTextFacet
    },
    com_atproto::label::Label
};

/*    Type: listViewBasic
    Id: app.bsky.graph.defs#listViewBasic
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - cid: string (JsonProperty: cid) [Required]
    - name: string (JsonProperty: name) [Required]
    - purpose: #listPurpose (JsonProperty: purpose) [Required]
    - avatar: string (JsonProperty: avatar) [Optional]
    - list_item_count: integer  (JsonProperty: listItemCount) [Optional]
    - labels: com.atproto.label.defs#label[] (JsonProperty: labels) [Optional]
    - viewer: #listViewerState (JsonProperty: viewer) [Optional]
    - indexed_at: datetime (JsonProperty: indexedAt) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct ListViewBasic {
    uri: String,
    cid: String,
    name: String,
    purpose: String,
    avatar: Option<String>,
    list_item_count: i32,
    labels: Option<Vec<Label>>,
    viewer: Option<ListViewerState>,
    indexed_at: Option<DateTime<Utc>>
}

/*    Type: listView
    Id: app.bsky.graph.defs#listView
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - cid: string (JsonProperty: cid) [Required]
    - creator: app.bsky.actor.defs#profileView (JsonProperty: creator) [Required]
    - name: string (JsonProperty: name) [Required]
    - purpose: #listPurpose (JsonProperty: purpose) [Required]
    - description: string (JsonProperty: description) [Optional]
    - description_facets: app.bsky.richtext.facet[] (JsonProperty: descriptionFacets) [Optional]
    - avatar: string (JsonProperty: avatar) [Optional]
    - list_item_count: integer  (JsonProperty: listItemCount) [Optional]
    - labels: com.atproto.label.defs#label[] (JsonProperty: labels) [Optional]
    - viewer: #listViewerState (JsonProperty: viewer) [Optional]
    - indexed_at: datetime (JsonProperty: indexedAt) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.graph.defs#listView")]
pub struct ListView {
    uri: String,
    cid: String,
    creator: ProfileView,
    name: String,
    purpose: String,
    description: Option<String>,
    description_facets: Option<Vec<RichTextFacet>>,
    avatar: Option<String>,
    list_item_count: i32,
    labels: Option<Vec<Label>>,
    viewer: Option<ListViewerState>,
    indexed_at: DateTime<Utc>
}

/*    Type: listItemView
    Id: app.bsky.graph.defs#listItemView
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - subject: app.bsky.actor.defs#profileView (JsonProperty: subject) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct ListItemView {
    uri: String,
    subject: ProfileView
}

/*    Type: starterPackView
    Id: app.bsky.graph.defs#starterPackView
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - cid: string (JsonProperty: cid) [Required]
    - record: unknown  (JsonProperty: record) [Required]
    - creator: app.bsky.actor.defs#profileViewBasic (JsonProperty: creator) [Required]
    - list: #listViewBasic (JsonProperty: list) [Optional]
    - list_items_sample: #listItemView[] (JsonProperty: listItemsSample) [Optional]
    - feeds: app.bsky.feed.defs#generatorView[] (JsonProperty: feeds) [Optional]
    - joined_week_count: integer  (JsonProperty: joinedWeekCount) [Optional]
    - joined_all_time_count: integer  (JsonProperty: joinedAllTimeCount) [Optional]
    - labels: com.atproto.label.defs#label[] (JsonProperty: labels) [Optional]
    - indexed_at: datetime (JsonProperty: indexedAt) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct StarterPackView {
    uri: String,
    cid: String,
    record: serde_json::Value,
    creator: ProfileViewBasic,
    list: Option<ListViewBasic>,
    list_items_sample: Option<Vec<ListItemView>>,
    feeds: Option<Vec<GeneratorView>>,
    joined_week_count: i32,
    joined_all_time_count: i32,
    labels: Option<Vec<Label>>,
    indexed_at: DateTime<Utc>
}

/*    Type: starterPackViewBasic
    Id: app.bsky.graph.defs#starterPackViewBasic
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - cid: string (JsonProperty: cid) [Required]
    - record: unknown  (JsonProperty: record) [Required]
    - creator: app.bsky.actor.defs#profileViewBasic (JsonProperty: creator) [Required]
    - list_item_count: integer  (JsonProperty: listItemCount) [Optional]
    - joined_week_count: integer  (JsonProperty: joinedWeekCount) [Optional]
    - joined_all_time_count: integer  (JsonProperty: joinedAllTimeCount) [Optional]
    - labels: com.atproto.label.defs#label[] (JsonProperty: labels) [Optional]
    - indexed_at: datetime (JsonProperty: indexedAt) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.graph.defs#starterPackViewBasic")]
pub struct StarterPackViewBasic {
    uri: String,
    cid: String,
    record: serde_json::Value,
    creator: ProfileViewBasic,
    list_item_count: i32,
    joined_week_count: i32,
    joined_all_time_count: i32,
    labels: Option<Vec<Label>>,
    indexed_at: DateTime<Utc>
}

/*    Type: listViewerState
    Id: app.bsky.graph.defs#listViewerState
    Kind: object

    Properties:
    - muted: boolean  (JsonProperty: muted) [Optional]
    - blocked: string (JsonProperty: blocked) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.graph.defs#listViewerState")]
pub struct ListViewerState {
    muted: bool,
    blocked: Option<String>
}

/*    Type: notFoundActor
    Id: app.bsky.graph.defs#notFoundActor
    Kind: object

    Properties:
    - actor: string (JsonProperty: actor) [Required]
    - not_found: boolean  (JsonProperty: notFound) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.graph.defs#notFoundActor")]
pub struct NotFoundActor {
    actor: String,
    not_found: bool
}

/*    Type: relationship
    Id: app.bsky.graph.defs#relationship
    Kind: object

    Properties:
    - did: string (JsonProperty: did) [Required]
    - following: string (JsonProperty: following) [Optional]
    - followed_by: string (JsonProperty: followedBy) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.graph.defs#relationship")]
pub struct Relationship {
    did: String,
    following: Option<String>,
    followed_by: Option<String>
}
