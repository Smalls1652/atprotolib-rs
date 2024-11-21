use serde::{Deserialize, Serialize};

/*
    com.atproto.identity.submitPlcOperation
*/

/*    Type: request
    Id: com.atproto.identity.submitPlcOperation#request
    Kind: object
    
    Properties:
    - operation: unknown  (JsonProperty: operation) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct SubmitPlcOperationRequest {
    #[serde(rename = "operation")]
    pub operation: serde_json::Value
}
