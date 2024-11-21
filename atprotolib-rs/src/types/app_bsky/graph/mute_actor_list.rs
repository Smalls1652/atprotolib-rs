use serde::{Deserialize, Serialize};

/*
    app.bsky.graph.muteActorList
*/

/*    Type: request
    Id: app.bsky.graph.muteActorList#request
    Kind: object
    
    Properties:
    - list: string (JsonProperty: list) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct MuteActorListRequest {
    list: String
}
