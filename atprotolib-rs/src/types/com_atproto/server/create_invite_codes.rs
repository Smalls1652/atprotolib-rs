use serde::{Deserialize, Serialize};

/*
    com.atproto.server.createInviteCodes
*/

/*    Type: accountCodes
    Id: com.atproto.server.createInviteCodes#accountCodes
    Kind: object
    
    Properties:
    - account: string (JsonProperty: account) [Required]
    - codes: string[] (JsonProperty: codes) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct AccountCodes {
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "codes")]
    pub codes: Vec<String>
}
