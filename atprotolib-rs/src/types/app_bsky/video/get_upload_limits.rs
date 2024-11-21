use serde::{Deserialize, Serialize};

/*
    app.bsky.video.getUploadLimits
*/

/*    Type: response
    Id: app.bsky.video.getUploadLimits#response
    Kind: object
    
    Properties:
    - can_upload: boolean  (JsonProperty: canUpload) [Required]
    - remaining_daily_videos: integer  (JsonProperty: remainingDailyVideos) [Optional]
    - remaining_daily_bytes: integer  (JsonProperty: remainingDailyBytes) [Optional]
    - message: string (JsonProperty: message) [Optional]
    - error: string (JsonProperty: error) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetUploadLimitsResponse {
    #[serde(rename = "canUpload")]
    pub can_upload: bool,
    #[serde(rename = "remainingDailyVideos")]
    pub remaining_daily_videos: i32,
    #[serde(rename = "remainingDailyBytes")]
    pub remaining_daily_bytes: i32,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>
}
