use serde::{Deserialize, Serialize};

use super::defs::ListView;

/*
    app.bsky.graph.getLists
*/

/*    Type: response
    Id: app.bsky.graph.getLists#response
    Kind: object

    Properties:
    - cursor: string (JsonProperty: cursor) [Optional]
    - lists: app.bsky.graph.defs#listView[] (JsonProperty: lists) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetListsResponse {
    cursor: Option<String>,
    lists: Vec<ListView>
}
