use serde::{Deserialize, Serialize};

use crate::types::app_bsky::actor::ProfileView;

/*
    app.bsky.graph.getMutes
*/

/*    Type: response
    Id: app.bsky.graph.getMutes#response
    Kind: object

    Properties:
    - cursor: string (JsonProperty: cursor) [Optional]
    - mutes: app.bsky.actor.defs#profileView[] (JsonProperty: mutes) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetMutesResponse {
    cursor: Option<String>,
    mutes: Vec<ProfileView>
}
