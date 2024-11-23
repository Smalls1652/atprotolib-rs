use serde::{Deserialize, Serialize};

use super::defs::ListView;

/*
    app.bsky.graph.getListBlocks
*/

/*    Type: response
    Id: app.bsky.graph.getListBlocks#response
    Kind: object

    Properties:
    - cursor: string (JsonProperty: cursor) [Optional]
    - lists: app.bsky.graph.defs#listView[] (JsonProperty: lists) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetListBlocksResponse {
    cursor: Option<String>,
    lists: Vec<ListView>
}
