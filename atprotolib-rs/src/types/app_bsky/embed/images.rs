use serde::{Deserialize, Serialize};

use super::AspectRatio;

/*
    app.bsky.embed.images
*/

/*    Type: image
    Id: app.bsky.embed.images#image
    Kind: object

    Properties:
    - image: blob  (JsonProperty: image) [Required]
    - alt: string (JsonProperty: alt) [Required]
    - aspect_ratio: app.bsky.embed.defs#aspectRatio (JsonProperty: aspectRatio) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageEmbed {
    #[serde(rename = "image")]
    pub image: Vec<u8>,
    #[serde(rename = "alt")]
    pub alt: String,
    #[serde(rename = "aspectRatio", skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<AspectRatio>
}

/*    Type: view
    Id: app.bsky.embed.images#view
    Kind: object

    Properties:
    - images: #viewImage[] (JsonProperty: images) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageEmbedView {
    #[serde(rename = "images")]
    pub images: Vec<ImageEmbed>
}

/*    Type: viewImage
    Id: app.bsky.embed.images#viewImage
    Kind: object

    Properties:
    - thumb: string (JsonProperty: thumb) [Required]
    - fullsize: string (JsonProperty: fullsize) [Required]
    - alt: string (JsonProperty: alt) [Required]
    - aspect_ratio: app.bsky.embed.defs#aspectRatio (JsonProperty: aspectRatio) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageEmbedViewImage {
    #[serde(rename = "thumb")]
    pub thumb: String,
    #[serde(rename = "fullsize")]
    pub fullsize: String,
    #[serde(rename = "alt")]
    pub alt: String,
    #[serde(rename = "aspectRatio", skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<AspectRatio>
}
