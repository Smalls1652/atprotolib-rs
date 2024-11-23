use serde::{Deserialize, Serialize};

/*
    app.bsky.graph.muteThread
*/

/*    Type: request
    Id: app.bsky.graph.muteThread#request
    Kind: object

    Properties:
    - root: string (JsonProperty: root) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct MuteThreadRequest {
    root: String
}
