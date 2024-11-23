use serde::{Deserialize, Serialize};

/*
    app.bsky.graph.unmuteActorList
*/

/*    Type: request
    Id: app.bsky.graph.unmuteActorList#request
    Kind: object

    Properties:
    - list: string (JsonProperty: list) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct UnmuteActorListRequest {
    list: String
}
