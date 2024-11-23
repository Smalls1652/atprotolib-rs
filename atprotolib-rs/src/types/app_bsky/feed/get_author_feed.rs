use serde::{Deserialize, Serialize};

use super::defs::FeedViewPost;

/*
    app.bsky.feed.getAuthorFeed
*/

/*    Type: response
    Id: app.bsky.feed.getAuthorFeed#response
    Kind: object

    Properties:
    - cursor: string (JsonProperty: cursor) [Optional]
    - feed: app.bsky.feed.defs#feedViewPost[] (JsonProperty: feed) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetAuthorFeedResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "feed")]
    pub feed: Vec<FeedViewPost>
}
