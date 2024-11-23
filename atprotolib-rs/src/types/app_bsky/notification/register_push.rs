use serde::{Deserialize, Serialize};

/*
    app.bsky.notification.registerPush
*/

/*    Type: request
    Id: app.bsky.notification.registerPush#request
    Kind: object

    Properties:
    - service_did: string (JsonProperty: serviceDid) [Required]
    - token: string (JsonProperty: token) [Required]
    - platform: string (JsonProperty: platform) [Required]
    - app_id: string (JsonProperty: appId) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterPushRequest {
    #[serde(rename = "serviceDid")]
    pub service_did: String,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "appId")]
    pub app_id: String
}
