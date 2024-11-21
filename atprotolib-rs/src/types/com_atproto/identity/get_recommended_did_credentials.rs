use serde::{Deserialize, Serialize};

/*
    com.atproto.identity.getRecommendedDidCredentials
*/

/*    Type: response
    Id: com.atproto.identity.getRecommendedDidCredentials#response
    Kind: object
    
    Properties:
    - rotation_keys: string[] (JsonProperty: rotationKeys) [Optional]
    - also_known_as: string[] (JsonProperty: alsoKnownAs) [Optional]
    - verification_methods: unknown  (JsonProperty: verificationMethods) [Optional]
    - services: unknown  (JsonProperty: services) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetRecommendedDidCredentialsResponse {
    #[serde(rename = "rotationKeys")]
    pub rotation_keys: Vec<String>,
    #[serde(rename = "alsoKnownAs")]
    pub also_known_as: Vec<String>,
    #[serde(rename = "verificationMethods")]
    pub verification_methods: serde_json::Value,
    #[serde(rename = "services")]
    pub services: serde_json::Value
}
