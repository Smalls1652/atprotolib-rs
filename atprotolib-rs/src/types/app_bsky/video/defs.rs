use serde::{Deserialize, Serialize};

/*    Type: jobStatus
    Id: app.bsky.video.defs#jobStatus
    Kind: object
    
    Properties:
    - job_id: string (JsonProperty: jobId) [Required]
    - did: string (JsonProperty: did) [Required]
    - state: string (JsonProperty: state) [Required]
    - progress: integer  (JsonProperty: progress) [Optional]
    - blob: blob  (JsonProperty: blob) [Optional]
    - error: string (JsonProperty: error) [Optional]
    - message: string (JsonProperty: message) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.video.defs#jobStatus")]
pub struct JobStatus {
    #[serde(rename = "jobId")]
    pub job_id: String,
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "progress")]
    pub progress: i32,
    #[serde(rename = "blob", skip_serializing_if = "Option::is_none")]
    pub blob: Option<Vec<u8>>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>
}
