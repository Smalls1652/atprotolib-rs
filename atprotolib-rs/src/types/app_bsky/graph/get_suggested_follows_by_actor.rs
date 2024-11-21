use serde::{Deserialize, Serialize};

use crate::types::app_bsky::actor::ProfileView;

/*
    app.bsky.graph.getSuggestedFollowsByActor
*/

/*    Type: response
    Id: app.bsky.graph.getSuggestedFollowsByActor#response
    Kind: object
    
    Properties:
    - suggestions: app.bsky.actor.defs#profileView[] (JsonProperty: suggestions) [Required]
    - is_fallback: boolean  (JsonProperty: isFallback) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetSuggestedFollowsByActorResponse {
    suggestions: Vec<ProfileView>,
    is_fallback: bool
}
