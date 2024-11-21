use serde::{Deserialize, Serialize};

use super::ProfileViewDetailed;

/*
    app.bsky.actor.getProfiles
*/

/*    Type: response
    Id: app.bsky.actor.getProfiles#response
    Kind: object
    
    Properties:
    - profiles: app.bsky.actor.defs#profileViewDetailed[] (JsonProperty: profiles) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetProfilesResponse {
    #[serde(rename = "profiles")]
    pub profiles: Vec<ProfileViewDetailed>
}
