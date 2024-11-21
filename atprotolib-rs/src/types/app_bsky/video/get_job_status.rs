use serde::{Deserialize, Serialize};

use super::JobStatus;

/*
    app.bsky.video.getJobStatus
*/

/*    Type: response
    Id: app.bsky.video.getJobStatus#response
    Kind: object
    
    Properties:
    - job_status: app.bsky.video.defs#jobStatus (JsonProperty: jobStatus) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetJobStatusResponse {
    #[serde(rename = "jobStatus")]
    pub job_status: JobStatus
}
