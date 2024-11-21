use serde::{Deserialize, Serialize};

/*
    app.bsky.richtext.facet
*/

/*
    Type: RichTextFacet
    Id: app.bsky.richtext.facet#main
    Kind: object

    Properties:
    - index: #byteSlice (JsonProperty: index) [Required]
    - features: union[] (JsonProperty: features) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.richtext.facet")]
pub struct RichTextFacet {
    #[serde(rename = "index")]
    index: ByteSlice,
    #[serde(rename = "features")]
    features: Vec<RichTextFacetFeatures>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RichTextFacetFeatures {
    Mention(Mention),
    Link(Link),
    Tag(Tag)
}

/*    Type: mention
    Id: app.bsky.richtext.facet#mention
    Kind: object
    
    Properties:
    - did: string (JsonProperty: did) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.richtext.facet#mention")]
pub struct Mention {
    #[serde(rename = "did")]
    did: String
}

/*    Type: link
    Id: app.bsky.richtext.facet#link
    Kind: object
    
    Properties:
    - uri: string (JsonProperty: uri) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.richtext.facet#link")]
pub struct Link {
    #[serde(rename = "uri")]
    uri: String
}

/*    Type: tag
    Id: app.bsky.richtext.facet#tag
    Kind: object
    
    Properties:
    - tag: string (JsonProperty: tag) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.richtext.facet#tag")]
pub struct Tag {
    #[serde(rename = "tag")]
    tag: String
}

/*    Type: byteSlice
    Id: app.bsky.richtext.facet#byteSlice
    Kind: object
    
    Properties:
    - byte_start: integer  (JsonProperty: byteStart) [Required]
    - byte_end: integer  (JsonProperty: byteEnd) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.richtext.facet#byteSlice")]
pub struct ByteSlice {
    #[serde(rename = "byteStart")]
    byte_start: i64,
    #[serde(rename = "byteEnd")]
    byte_end: i64
}
