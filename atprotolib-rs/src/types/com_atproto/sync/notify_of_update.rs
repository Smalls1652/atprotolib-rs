use serde::{Deserialize, Serialize};

/*
    com.atproto.sync.notifyOfUpdate
*/

/*    Type: request
    Id: com.atproto.sync.notifyOfUpdate#request
    Kind: object
    
    Properties:
    - hostname: string (JsonProperty: hostname) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct NotifyOfUpdateRequest {
    #[serde(rename = "hostname")]
    pub hostname: String
}
