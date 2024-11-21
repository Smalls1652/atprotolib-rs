use serde::{Deserialize, Serialize};

use super::JobStatus;

/*
    app.bsky.video.uploadVideo
*/

/*    Type: response
    Id: app.bsky.video.uploadVideo#response
    Kind: object
    
    Properties:
    - job_status: app.bsky.video.defs#jobStatus (JsonProperty: jobStatus) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct UploadVideoResponse {
    #[serde(rename = "jobStatus")]
    pub job_status: JobStatus
}
