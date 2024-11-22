use serde::{Deserialize, Serialize};

/*
    app.bsky.notification.getUnreadCount
*/

/*    Type: response
    Id: app.bsky.notification.getUnreadCount#response
    Kind: object
    
    Properties:
    - count: integer  (JsonProperty: count) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetUnreadCountResponse {
    #[serde(rename = "count", default)]
    pub count: i32
}
