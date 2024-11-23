use serde::{Deserialize, Serialize};

/*
    app.bsky.graph.unmuteThread
*/

/*    Type: request
    Id: app.bsky.graph.unmuteThread#request
    Kind: object

    Properties:
    - root: string (JsonProperty: root) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct UnmuteThreadRequest {
    root: String
}
