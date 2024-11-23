use serde::{Deserialize, Serialize};

/*
    app.bsky.graph.muteActor
*/

/*    Type: request
    Id: app.bsky.graph.muteActor#request
    Kind: object

    Properties:
    - actor: string (JsonProperty: actor) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct MuteActorRequest {
    actor: String
}
