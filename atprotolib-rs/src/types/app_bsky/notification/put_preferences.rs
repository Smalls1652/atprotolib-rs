use serde::{Deserialize, Serialize};

/*
    app.bsky.notification.putPreferences
*/

/*    Type: request
    Id: app.bsky.notification.putPreferences#request
    Kind: object

    Properties:
    - priority: boolean  (JsonProperty: priority) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct PutPreferencesRequest {
    #[serde(rename = "priority", default)]
    pub priority: bool
}
