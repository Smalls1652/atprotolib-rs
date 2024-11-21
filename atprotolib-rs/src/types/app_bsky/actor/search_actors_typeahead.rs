use serde::{Deserialize, Serialize};

use super::ProfileViewBasic;

/*
    app.bsky.actor.searchActorsTypeahead
*/

/*    Type: response
    Id: app.bsky.actor.searchActorsTypeahead#response
    Kind: object
    
    Properties:
    - actors: app.bsky.actor.defs#profileViewBasic[] (JsonProperty: actors) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchActorsTypeaheadResponse {
    #[serde(rename = "actors")]
    pub actors: Vec<ProfileViewBasic>
}


