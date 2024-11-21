use serde::{Deserialize, Serialize};

use super::defs::StarterPackViewBasic;

/*
    app.bsky.graph.getStarterPacks
*/

/*    Type: response
    Id: app.bsky.graph.getStarterPacks#response
    Kind: object
    
    Properties:
    - starter_packs: app.bsky.graph.defs#starterPackViewBasic[] (JsonProperty: starterPacks) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetStarterPacksResponse {
    starter_packs: Vec<StarterPackViewBasic>
}
