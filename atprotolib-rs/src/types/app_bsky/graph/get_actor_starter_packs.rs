use serde::{Deserialize, Serialize};

use super::defs::StarterPackViewBasic;

/*
    app.bsky.graph.getActorStarterPacks
*/

/*    Type: response
    Id: app.bsky.graph.getActorStarterPacks#response
    Kind: object
    
    Properties:
    - cursor: string (JsonProperty: cursor) [Optional]
    - starter_packs: app.bsky.graph.defs#starterPackViewBasic[] (JsonProperty: starterPacks) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetActorStarterPacksResponse {
    cursor: Option<String>,
    starter_packs: Vec<StarterPackViewBasic>
}

