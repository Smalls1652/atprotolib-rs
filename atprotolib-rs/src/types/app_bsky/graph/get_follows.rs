use serde::{Deserialize, Serialize};

use crate::types::app_bsky::actor::ProfileView;

/*
    app.bsky.graph.getFollows
*/

/*    Type: response
    Id: app.bsky.graph.getFollows#response
    Kind: object
    
    Properties:
    - subject: app.bsky.actor.defs#profileView (JsonProperty: subject) [Required]
    - cursor: string (JsonProperty: cursor) [Optional]
    - follows: app.bsky.actor.defs#profileView[] (JsonProperty: follows) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetFollowsResponse {
    subject: ProfileView,
    cursor: Option<String>,
    follows: Vec<ProfileView>
}
