use serde::{Deserialize, Serialize};

/*
    app.bsky.richtext.facet
*/

/// Represents a facet of rich text.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.richtext.facet")]
pub struct RichTextFacet {
    /// The index of the facet in the rich text.
    #[serde(rename = "index")]
    index: ByteSlice,

    /// The features of the facet.
    #[serde(rename = "features")]
    features: Vec<RichTextFacetFeatures>
}

/// A type union for the features of a rich text facet.
#[derive(Serialize, Deserialize, Debug)]
pub enum RichTextFacetFeatures {
    /// A mention.
    Mention(Mention),

    /// A link.
    Link(Link),

    /// A tag.
    Tag(Tag)
}

/// Represents a mention in a rich text facet.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.richtext.facet#mention")]
pub struct Mention {
    /// The DID of the mention.
    #[serde(rename = "did")]
    did: String
}

/// Represents a link in a rich text facet.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.richtext.facet#link")]
pub struct Link {
    /// The URI of the link.
    #[serde(rename = "uri")]
    uri: String
}

/// Represents a tag in a rich text facet.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.richtext.facet#tag")]
pub struct Tag {
    /// The tag.
    #[serde(rename = "tag")]
    tag: String
}

/// Represents a byte slice.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.richtext.facet#byteSlice")]
pub struct ByteSlice {
    /// The start byte.
    #[serde(rename = "byteStart")]
    byte_start: i64,

    /// The end byte.
    #[serde(rename = "byteEnd")]
    byte_end: i64
}
