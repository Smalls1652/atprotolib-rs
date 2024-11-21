use serde::{Deserialize, Serialize};

use super::defs::ListView;

/*
    app.bsky.graph.getListMutes
*/

/*    Type: response
    Id: app.bsky.graph.getListMutes#response
    Kind: object
    
    Properties:
    - cursor: string (JsonProperty: cursor) [Optional]
    - lists: app.bsky.graph.defs#listView[] (JsonProperty: lists) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetListMutesResponse {
    cursor: Option<String>,
    lists: Vec<ListView>
}
