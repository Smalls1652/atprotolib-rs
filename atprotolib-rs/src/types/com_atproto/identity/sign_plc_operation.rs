use serde::{Deserialize, Serialize};

/*
    com.atproto.identity.signPlcOperation
*/

/*    Type: request
    Id: com.atproto.identity.signPlcOperation#request
    Kind: object

    Properties:
    - token: string (JsonProperty: token) [Optional]
    - rotation_keys: string[] (JsonProperty: rotationKeys) [Optional]
    - also_known_as: string[] (JsonProperty: alsoKnownAs) [Optional]
    - verification_methods: unknown  (JsonProperty: verificationMethods) [Optional]
    - services: unknown  (JsonProperty: services) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct SignPlcOperationRequest {
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "rotationKeys", skip_serializing_if = "Option::is_none")]
    pub rotation_keys: Option<Vec<String>>,
    #[serde(rename = "alsoKnownAs", skip_serializing_if = "Option::is_none")]
    pub also_known_as: Option<Vec<String>>,
    #[serde(
        rename = "verificationMethods",
        skip_serializing_if = "Option::is_none"
    )]
    pub verification_methods: Option<serde_json::Value>,
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<serde_json::Value>
}

/*    Type: response
    Id: com.atproto.identity.signPlcOperation#response
    Kind: object

    Properties:
    - operation: unknown  (JsonProperty: operation) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct SignPlcOperationResponse {
    #[serde(rename = "operation")]
    pub operation: serde_json::Value
}
