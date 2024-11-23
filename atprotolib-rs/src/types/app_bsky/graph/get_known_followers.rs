use serde::{Deserialize, Serialize};

use crate::types::app_bsky::actor::ProfileView;

/*
    app.bsky.graph.getKnownFollowers
*/

/*    Type: response
    Id: app.bsky.graph.getKnownFollowers#response
    Kind: object

    Properties:
    - subject: app.bsky.actor.defs#profileView (JsonProperty: subject) [Required]
    - cursor: string (JsonProperty: cursor) [Optional]
    - followers: app.bsky.actor.defs#profileView[] (JsonProperty: followers) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetKnownFollowersResponse {
    subject: ProfileView,
    cursor: Option<String>,
    followers: Vec<ProfileView>
}
