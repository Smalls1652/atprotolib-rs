use serde::{Deserialize, Serialize};

/*
    com.atproto.identity.resolveHandle
*/

/*    Type: response
    Id: com.atproto.identity.resolveHandle#response
    Kind: object

    Properties:
    - did: string (JsonProperty: did) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct ResolveHandleResponse {
    #[serde(rename = "did")]
    pub did: String
}
