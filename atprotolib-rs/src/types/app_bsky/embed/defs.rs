use serde::{Deserialize, Serialize};

/*    Type: aspectRatio
    Id: app.bsky.embed.defs#aspectRatio
    Kind: object
    
    Properties:
    - width: integer  (JsonProperty: width) [Required]
    - height: integer  (JsonProperty: height) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.embed.defs#aspectRatio")]
pub struct AspectRatio {
    #[serde(rename = "width")]
    pub width: i32,
    #[serde(rename = "height")]
    pub height: i32,
}
