use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{app_bsky::graph::{ListViewBasic, StarterPackViewBasic}, com_atproto::label::Label};

/*    Type: profileViewBasic
    Id: app.bsky.actor.defs#profileViewBasic
    Kind: object
    
    Properties:
    - did: string (JsonProperty: did) [Required]
    - handle: string (JsonProperty: handle) [Required]
    - display_name: string (JsonProperty: displayName) [Optional]
    - avatar: string (JsonProperty: avatar) [Optional]
    - associated: #profileAssociated (JsonProperty: associated) [Optional]
    - viewer: #viewerState (JsonProperty: viewer) [Optional]
    - labels: com.atproto.label.defs#label[] (JsonProperty: labels) [Optional]
    - created_at: datetime (JsonProperty: createdAt) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#profileViewBasic")]
pub struct ProfileViewBasic {
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "handle")]
    pub handle: String,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(rename = "associated", skip_serializing_if = "Option::is_none")]
    pub associated: Option<ProfileAssociated>,
    #[serde(rename = "viewer", skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerState>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>
}

/*    Type: profileView
    Id: app.bsky.actor.defs#profileView
    Kind: object
    
    Properties:
    - did: string (JsonProperty: did) [Required]
    - handle: string (JsonProperty: handle) [Required]
    - display_name: string (JsonProperty: displayName) [Optional]
    - description: string (JsonProperty: description) [Optional]
    - avatar: string (JsonProperty: avatar) [Optional]
    - associated: #profileAssociated (JsonProperty: associated) [Optional]
    - indexed_at: datetime (JsonProperty: indexedAt) [Optional]
    - created_at: datetime (JsonProperty: createdAt) [Optional]
    - viewer: #viewerState (JsonProperty: viewer) [Optional]
    - labels: com.atproto.label.defs#label[] (JsonProperty: labels) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#profileView")]
pub struct ProfileView {
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "handle")]
    pub handle: String,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(rename = "associated", skip_serializing_if = "Option::is_none")]
    pub associated: Option<ProfileAssociated>,
    #[serde(rename = "indexedAt", skip_serializing_if = "Option::is_none")]
    pub indexed_at: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "viewer", skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerState>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>
}

/*    Type: profileViewDetailed
    Id: app.bsky.actor.defs#profileViewDetailed
    Kind: object
    
    Properties:
    - did: string (JsonProperty: did) [Required]
    - handle: string (JsonProperty: handle) [Required]
    - display_name: string (JsonProperty: displayName) [Optional]
    - description: string (JsonProperty: description) [Optional]
    - avatar: string (JsonProperty: avatar) [Optional]
    - banner: string (JsonProperty: banner) [Optional]
    - followers_count: integer  (JsonProperty: followersCount) [Optional]
    - follows_count: integer  (JsonProperty: followsCount) [Optional]
    - posts_count: integer  (JsonProperty: postsCount) [Optional]
    - associated: #profileAssociated (JsonProperty: associated) [Optional]
    - joined_via_starter_pack: app.bsky.graph.defs#starterPackViewBasic (JsonProperty: joinedViaStarterPack) [Optional]
    - indexed_at: datetime (JsonProperty: indexedAt) [Optional]
    - created_at: datetime (JsonProperty: createdAt) [Optional]
    - viewer: #viewerState (JsonProperty: viewer) [Optional]
    - labels: com.atproto.label.defs#label[] (JsonProperty: labels) [Optional]
    - pinned_post: com.atproto.repo.strongRef (JsonProperty: pinnedPost) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#profileViewDetailed")]
pub struct ProfileViewDetailed {
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "handle")]
    pub handle: String,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(rename = "banner", skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    #[serde(rename = "followersCount")]
    pub followers_count: i32,
    #[serde(rename = "followsCount")]
    pub follows_count: i32,
    #[serde(rename = "postsCount")]
    pub posts_count: i32,
    #[serde(rename = "associated", skip_serializing_if = "Option::is_none")]
    pub associated: Option<ProfileAssociated>,
    #[serde(rename = "joinedViaStarterPack", skip_serializing_if = "Option::is_none")]
    pub joined_via_starter_pack: Option<StarterPackViewBasic>,
    #[serde(rename = "indexedAt", skip_serializing_if = "Option::is_none")]
    pub indexed_at: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "viewer", skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerState>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
    #[serde(rename = "pinnedPost", skip_serializing_if = "Option::is_none")]
    pub pinned_post: Option<String>
}

/*    Type: profileAssociated
    Id: app.bsky.actor.defs#profileAssociated
    Kind: object
    
    Properties:
    - lists: integer  (JsonProperty: lists) [Optional]
    - feedgens: integer  (JsonProperty: feedgens) [Optional]
    - starter_packs: integer  (JsonProperty: starterPacks) [Optional]
    - labeler: boolean  (JsonProperty: labeler) [Optional]
    - chat: #profileAssociatedChat (JsonProperty: chat) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#profileAssociated")]
pub struct ProfileAssociated {
    #[serde(rename = "lists")]
    pub lists: i32,
    #[serde(rename = "feedgens")]
    pub feedgens: i32,
    #[serde(rename = "starterPacks")]
    pub starter_packs: i32,
    #[serde(rename = "labeler")]
    pub labeler: bool,
    #[serde(rename = "chat", skip_serializing_if = "Option::is_none")]
    pub chat: Option<ProfileAssociatedChat>
}

/*    Type: profileAssociatedChat
    Id: app.bsky.actor.defs#profileAssociatedChat
    Kind: object
    
    Properties:
    - allow_incoming: string (JsonProperty: allowIncoming) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#profileAssociatedChat")]
pub struct ProfileAssociatedChat {
    #[serde(rename = "allowIncoming")]
    pub allow_incoming: String
}

/*    Type: viewerState
    Id: app.bsky.actor.defs#viewerState
    Kind: object
    
    Properties:
    - muted: boolean  (JsonProperty: muted) [Optional]
    - muted_by_list: app.bsky.graph.defs#listViewBasic (JsonProperty: mutedByList) [Optional]
    - blocked_by: boolean  (JsonProperty: blockedBy) [Optional]
    - blocking: string (JsonProperty: blocking) [Optional]
    - blocking_by_list: app.bsky.graph.defs#listViewBasic (JsonProperty: blockingByList) [Optional]
    - following: string (JsonProperty: following) [Optional]
    - followed_by: string (JsonProperty: followedBy) [Optional]
    - known_followers: #knownFollowers (JsonProperty: knownFollowers) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#viewerState")]
pub struct ViewerState {
    #[serde(rename = "muted")]
    pub muted: bool,
    #[serde(rename = "mutedByList", skip_serializing_if = "Option::is_none")]
    pub muted_by_list: Option<ListViewBasic>,
    #[serde(rename = "blockedBy")]
    pub blocked_by: bool,
    #[serde(rename = "blocking")]
    pub blocking: String,
    #[serde(rename = "blockingByList", skip_serializing_if = "Option::is_none")]
    pub blocking_by_list: Option<ListViewBasic>,
    #[serde(rename = "following")]
    pub following: String,
    #[serde(rename = "followedBy")]
    pub followed_by: String,
    #[serde(rename = "knownFollowers", skip_serializing_if = "Option::is_none")]
    pub known_followers: Option<KnownFollowers>
}

/*    Type: knownFollowers
    Id: app.bsky.actor.defs#knownFollowers
    Kind: object
    
    Properties:
    - count: integer  (JsonProperty: count) [Required]
    - followers: #profileViewBasic[] (JsonProperty: followers) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#knownFollowers")]
pub struct KnownFollowers {
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "followers")]
    pub followers: Vec<ProfileViewBasic>
}

/*    Type: preferences
    Id: app.bsky.actor.defs#preferences
    Kind: array union
    
    Properties:
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#preferences")]
pub struct Preferences {
    #[serde(rename = "preferences")]
    pub preferences: Vec<PreferencesEnum>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PreferencesEnum {
    AdultContentPref(AdultContentPref),
    ContentLabelPref(ContentLabelPref),
    SavedFeedsPrefV2(SavedFeedsPrefV2),
    SavedFeedsPref(SavedFeedsPref),
    PersonalDetailsPref(PersonalDetailsPref),
    FeedViewPref(FeedViewPref),
    ThreadViewPref(ThreadViewPref),
    InterestsPref(InterestsPref),
    MutedWordsPref(MutedWordsPref),
    HiddenPostsPref(HiddenPostsPref),
    LabelersPref(LabelersPref),
    BskyAppStatePref(BskyAppStatePref)
}

/*    Type: adultContentPref
    Id: app.bsky.actor.defs#adultContentPref
    Kind: object
    
    Properties:
    - enabled: boolean  (JsonProperty: enabled) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#adultContentPref")]
pub struct AdultContentPref {
    #[serde(rename = "enabled")]
    pub enabled: bool
}

/*    Type: contentLabelPref
    Id: app.bsky.actor.defs#contentLabelPref
    Kind: object
    
    Properties:
    - labeler_did: string (JsonProperty: labelerDid) [Optional]
    - label: string (JsonProperty: label) [Required]
    - visibility: string (JsonProperty: visibility) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#contentLabelPref")]
pub struct ContentLabelPref {
    #[serde(rename = "labelerDid", skip_serializing_if = "Option::is_none")]
    pub labeler_did: Option<String>,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "visibility")]
    pub visibility: String
}

/*    Type: savedFeed
    Id: app.bsky.actor.defs#savedFeed
    Kind: object
    
    Properties:
    - id: string (JsonProperty: id) [Required]
    - type: string (JsonProperty: type) [Required]
    - value: string (JsonProperty: value) [Required]
    - pinned: boolean  (JsonProperty: pinned) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "feed")]
pub struct SavedFeed {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "pinned")]
    pub pinned: bool
}

/*    Type: savedFeedsPrefV2
    Id: app.bsky.actor.defs#savedFeedsPrefV2
    Kind: object
    
    Properties:
    - items: app.bsky.actor.defs#savedFeed[] (JsonProperty: items) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#savedFeedsPrefV2")]
pub struct SavedFeedsPrefV2 {
    #[serde(rename = "items")]
    pub items: Vec<SavedFeed>
}

/*    Type: savedFeedsPref
    Id: app.bsky.actor.defs#savedFeedsPref
    Kind: object
    
    Properties:
    - pinned: string[] (JsonProperty: pinned) [Required]
    - saved: string[] (JsonProperty: saved) [Required]
    - timeline_index: integer  (JsonProperty: timelineIndex) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#savedFeedsPref")]
pub struct SavedFeedsPref {
    #[serde(rename = "pinned")]
    pub pinned: Vec<String>,
    #[serde(rename = "saved")]
    pub saved: Vec<String>,
    #[serde(rename = "timelineIndex")]
    pub timeline_index: i32
}

/*    Type: personalDetailsPref
    Id: app.bsky.actor.defs#personalDetailsPref
    Kind: object
    
    Properties:
    - birth_date: datetime (JsonProperty: birthDate) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#personalDetailsPref")]
pub struct PersonalDetailsPref {
    #[serde(rename = "birthDate", skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<DateTime<Utc>>
}

/*    Type: feedViewPref
    Id: app.bsky.actor.defs#feedViewPref
    Kind: object
    
    Properties:
    - feed: string (JsonProperty: feed) [Required]
    - hide_replies: boolean  (JsonProperty: hideReplies) [Optional]
    - hide_replies_by_unfollowed: boolean  (JsonProperty: hideRepliesByUnfollowed) [Optional]
    - hide_replies_by_like_count: integer  (JsonProperty: hideRepliesByLikeCount) [Optional]
    - hide_reposts: boolean  (JsonProperty: hideReposts) [Optional]
    - hide_quote_posts: boolean  (JsonProperty: hideQuotePosts) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#feedViewPref")]
pub struct FeedViewPref {
    #[serde(rename = "feed")]
    pub feed: String,
    #[serde(rename = "hideReplies")]
    pub hide_replies: bool,
    #[serde(rename = "hideRepliesByUnfollowed")]
    pub hide_replies_by_unfollowed: bool,
    #[serde(rename = "hideRepliesByLikeCount")]
    pub hide_replies_by_like_count: i32,
    #[serde(rename = "hideReposts")]
    pub hide_reposts: bool,
    #[serde(rename = "hideQuotePosts")]
    pub hide_quote_posts: bool
}

/*    Type: threadViewPref
    Id: app.bsky.actor.defs#threadViewPref
    Kind: object
    
    Properties:
    - sort: string (JsonProperty: sort) [Optional]
    - prioritize_followed_users: boolean  (JsonProperty: prioritizeFollowedUsers) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#threadViewPref")]
pub struct ThreadViewPref {
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(rename = "prioritizeFollowedUsers")]
    pub prioritize_followed_users: bool
}

/*    Type: interestsPref
    Id: app.bsky.actor.defs#interestsPref
    Kind: object
    
    Properties:
    - tags: string[] (JsonProperty: tags) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#interestsPref")]
pub struct InterestsPref {
    #[serde(rename = "tags")]
    pub tags: Vec<String>
}

/*    Type: mutedWord
    Id: app.bsky.actor.defs#mutedWord
    Kind: object
    
    Properties:
    - id: string (JsonProperty: id) [Optional]
    - value: string (JsonProperty: value) [Required]
    - targets: app.bsky.actor.defs#mutedWordTarget[] (JsonProperty: targets) [Required]
    - actor_target: string (JsonProperty: actorTarget) [Optional]
    - expires_at: datetime (JsonProperty: expiresAt) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#mutedWord")]
pub struct MutedWord {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "targets")]
    pub targets: Vec<String>,
    #[serde(rename = "actorTarget", skip_serializing_if = "Option::is_none")]
    pub actor_target: Option<String>,
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>
}

/*    Type: mutedWordsPref
    Id: app.bsky.actor.defs#mutedWordsPref
    Kind: object
    
    Properties:
    - items: app.bsky.actor.defs#mutedWord[] (JsonProperty: items) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#mutedWordsPref")]
pub struct MutedWordsPref {
    #[serde(rename = "items")]
    pub items: Vec<MutedWord>
}

/*    Type: hiddenPostsPref
    Id: app.bsky.actor.defs#hiddenPostsPref
    Kind: object
    
    Properties:
    - items: string[] (JsonProperty: items) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#hiddenPostsPref")]
pub struct HiddenPostsPref {
    #[serde(rename = "items")]
    pub items: Vec<String>
}

/*    Type: labelersPref
    Id: app.bsky.actor.defs#labelersPref
    Kind: object
    
    Properties:
    - labelers: #labelerPrefItem[] (JsonProperty: labelers) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#labelersPref")]
pub struct LabelersPref {
    #[serde(rename = "labelers")]
    pub labelers: Vec<LabelerPrefItem>
}

/*    Type: labelerPrefItem
    Id: app.bsky.actor.defs#labelerPrefItem
    Kind: object
    
    Properties:
    - did: string (JsonProperty: did) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#labelerPrefItem")]
pub struct LabelerPrefItem {
    #[serde(rename = "did")]
    pub did: String
}

/*    Type: bskyAppStatePref
    Id: app.bsky.actor.defs#bskyAppStatePref
    Kind: object
    
    Properties:
    - active_progress_guide: #bskyAppProgressGuide (JsonProperty: activeProgressGuide) [Optional]
    - queued_nudges: string[] (JsonProperty: queuedNudges) [Optional]
    - nuxs: app.bsky.actor.defs#nux[] (JsonProperty: nuxs) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#bskyAppStatePref")]
pub struct BskyAppStatePref {
    #[serde(rename = "activeProgressGuide", skip_serializing_if = "Option::is_none")]
    pub active_progress_guide: Option<BskyAppProgressGuide>,
    #[serde(rename = "queuedNudges", skip_serializing_if = "Option::is_none")]
    pub queued_nudges: Option<Vec<String>>,
    #[serde(rename = "nuxs", skip_serializing_if = "Option::is_none")]
    pub nuxs: Option<Vec<Nux>>
}

/*    Type: bskyAppProgressGuide
    Id: app.bsky.actor.defs#bskyAppProgressGuide
    Kind: object
    
    Properties:
    - guide: string (JsonProperty: guide) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#bskyAppProgressGuide")]
pub struct BskyAppProgressGuide {
    #[serde(rename = "guide")]
    pub guide: String
}

/*    Type: nux
    Id: app.bsky.actor.defs#nux
    Kind: object
    
    Properties:
    - id: string (JsonProperty: id) [Required]
    - completed: boolean  (JsonProperty: completed) [Required]
    - data: string (JsonProperty: data) [Optional]
    - expires_at: datetime (JsonProperty: expiresAt) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#nux")]
pub struct Nux {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "completed")]
    pub completed: bool,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>
}


