use serde::{Deserialize, Serialize};

use super::defs::GeneratorView;

/*
    app.bsky.feed.getActorFeeds
*/

/*    Type: response
    Id: app.bsky.feed.getActorFeeds#response
    Kind: object

    Properties:
    - cursor: string (JsonProperty: cursor) [Optional]
    - feeds: app.bsky.feed.defs#generatorView[] (JsonProperty: feeds) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetActorFeedsResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "feeds")]
    pub feeds: Vec<GeneratorView>
}
