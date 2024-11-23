use serde::{Deserialize, Serialize};

/*
    app.bsky.graph.unmuteActor
*/

/*    Type: request
    Id: app.bsky.graph.unmuteActor#request
    Kind: object

    Properties:
    - actor: string (JsonProperty: actor) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct UnmuteActorRequest {
    actor: String
}
