use serde::{Deserialize, Serialize};

/// Represents the status of a video upload job.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.video.defs#jobStatus")]
pub struct JobStatus {
    /// The ID of the job.
    #[serde(rename = "jobId")]
    pub job_id: String,

    /// The DID of the job.
    #[serde(rename = "did")]
    pub did: String,

    /// The state of the job.
    #[serde(rename = "state")]
    pub state: String,

    /// The progress of the job.
    #[serde(rename = "progress", default)]
    pub progress: i32,

    /// The blob of the job.
    #[serde(rename = "blob", skip_serializing_if = "Option::is_none")]
    pub blob: Option<Vec<u8>>,

    /// The error of the job.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// The message of the job.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>
}
