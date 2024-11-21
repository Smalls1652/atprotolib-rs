use serde::{Deserialize, Serialize};

/*
    com.atproto.identity.updateHandle
*/

/*    Type: request
    Id: com.atproto.identity.updateHandle#request
    Kind: object
    
    Properties:
    - handle: string (JsonProperty: handle) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateHandleRequest {
    #[serde(rename = "handle")]
    pub handle: String
}
