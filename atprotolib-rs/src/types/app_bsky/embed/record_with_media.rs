use serde::{Deserialize, Serialize};

use super::{ExternalEmbedView, ImageEmbedView, RecordEmbedView, VideoEmbedView};

/*
    app.bsky.embed.recordWithMedia
*/

/*    Type: view
    Id: app.bsky.embed.recordWithMedia#view
    Kind: object
    
    Properties:
    - record: app.bsky.embed.record#view (JsonProperty: record) [Required]
    - media: union  (JsonProperty: media) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.embed.recordWithMedia#view")]
pub struct RecordWithMediaEmbedView {
    #[serde(rename = "record")]
    pub record: RecordEmbedView,
    #[serde(rename = "media")]
    pub media: RecordWithMediaEmbedViewMedia
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RecordWithMediaEmbedViewMedia {
    Images(ImageEmbedView),
    Videos(VideoEmbedView),
    External(ExternalEmbedView)
}
