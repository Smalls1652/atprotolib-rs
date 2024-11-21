use serde::{Deserialize, Serialize};

use crate::types::app_bsky::actor::ProfileView;

/*
    app.bsky.graph.getBlocks
*/

/*    Type: response
    Id: app.bsky.graph.getBlocks#response
    Kind: object
    
    Properties:
    - cursor: string (JsonProperty: cursor) [Optional]
    - blocks: app.bsky.actor.defs#profileView[] (JsonProperty: blocks) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlocksResponse {
    cursor: Option<String>,
    blocks: Vec<ProfileView>
}
