use serde::{Deserialize, Serialize};

use super::defs::GeneratorView;

/*
    app.bsky.feed.getFeedGenerator
*/

/*    Type: response
    Id: app.bsky.feed.getFeedGenerator#response
    Kind: object
    
    Properties:
    - view: app.bsky.feed.defs#generatorView (JsonProperty: view) [Required]
    - is_online: boolean  (JsonProperty: isOnline) [Required]
    - is_valid: boolean  (JsonProperty: isValid) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetFeedGeneratorResponse {
    #[serde(rename = "view")]
    pub view: GeneratorView,
    #[serde(rename = "isOnline")]
    pub is_online: bool,
    #[serde(rename = "isValid")]
    pub is_valid: bool,
}
