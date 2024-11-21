use serde::{Deserialize, Serialize};

/*
    com.atproto.repo.uploadBlob
*/

/*    Type: response
    Id: com.atproto.repo.uploadBlob#response
    Kind: object
    
    Properties:
    - blob: blob  (JsonProperty: blob) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct UploadBlobResponse {
    #[serde(rename = "blob")]
    pub blob: Vec<u8>
}
