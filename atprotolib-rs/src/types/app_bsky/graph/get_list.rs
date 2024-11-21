use serde::{Deserialize, Serialize};

use super::defs::{ListItemView, ListView};

/*
    app.bsky.graph.getList
*/

/*    Type: response
    Id: app.bsky.graph.getList#response
    Kind: object
    
    Properties:
    - cursor: string (JsonProperty: cursor) [Optional]
    - list: app.bsky.graph.defs#listView (JsonProperty: list) [Required]
    - items: app.bsky.graph.defs#listItemView[] (JsonProperty: items) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetListResponse {
    cursor: Option<String>,
    list: ListView,
    items: Vec<ListItemView>
}
