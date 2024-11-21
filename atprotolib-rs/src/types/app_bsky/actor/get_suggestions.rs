use serde::{Deserialize, Serialize};

use super::ProfileView;

/*
    app.bsky.actor.getSuggestions
*/

/*    Type: response
    Id: app.bsky.actor.getSuggestions#response
    Kind: object
    
    Properties:
    - cursor: string (JsonProperty: cursor) [Optional]
    - actors: app.bsky.actor.defs#profileView[] (JsonProperty: actors) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetSuggestionsResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "actors")]
    pub actors: Vec<ProfileView>
}
