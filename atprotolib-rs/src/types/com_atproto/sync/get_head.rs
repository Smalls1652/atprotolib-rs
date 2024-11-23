use serde::{Deserialize, Serialize};

/*
    com.atproto.sync.getHead
*/

/*    Type: response
    Id: com.atproto.sync.getHead#response
    Kind: object

    Properties:
    - root: string (JsonProperty: root) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetHeadResponse {
    #[serde(rename = "root")]
    pub root: String
}
