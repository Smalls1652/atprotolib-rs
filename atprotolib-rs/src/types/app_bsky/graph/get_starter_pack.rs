use serde::{Deserialize, Serialize};

use super::defs::StarterPackView;

/*
    app.bsky.graph.getStarterPack
*/

/*    Type: response
    Id: app.bsky.graph.getStarterPack#response
    Kind: object

    Properties:
    - starter_pack: app.bsky.graph.defs#starterPackView (JsonProperty: starterPack) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetStarterPackResponse {
    starter_pack: StarterPackView
}
