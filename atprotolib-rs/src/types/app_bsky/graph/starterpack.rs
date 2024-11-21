use serde::{Deserialize, Serialize};

/*
    app.bsky.graph.starterpack
*/

/*    Type: feedItem
    Id: app.bsky.graph.starterpack#feedItem
    Kind: object
    
    Properties:
    - uri: string (JsonProperty: uri) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct StarterPackFeedItem {
    uri: String
}
