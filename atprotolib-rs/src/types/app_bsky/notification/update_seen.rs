use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/*
    app.bsky.notification.updateSeen
*/

/*    Type: request
    Id: app.bsky.notification.updateSeen#request
    Kind: object
    
    Properties:
    - seen_at: datetime (JsonProperty: seenAt) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateSeenRequest {
    #[serde(rename = "seenAt")]
    pub seen_at: DateTime<Utc>
}
