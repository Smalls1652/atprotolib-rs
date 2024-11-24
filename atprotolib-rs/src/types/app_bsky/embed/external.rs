use serde::{Deserialize, Serialize};

/*
    app.bsky.embed.external
*/

/// A representation of some externally linked content (eg, a URL and 'card'), embedded in a Bluesky record (eg, a post).
#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalEmbed {
    /// The URI of the external content.
    #[serde(rename = "uri")]
    pub uri: String,

    /// The title of the external content.
    #[serde(rename = "title")]
    pub title: String,

    /// A description of the external content.
    #[serde(rename = "description")]
    pub description: String,

    /// A thumbnail image representing the external content.
    #[serde(rename = "thumb", skip_serializing_if = "Option::is_none")]
    pub thumb: Option<Vec<u8>>
}

/// A view of an external embed.
#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalEmbedView {
    /// The external embed.
    #[serde(rename = "external")]
    pub external: ExternalEmbed
}

/// A view of an external embed, with the external embed itself embedded. (?)
#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalEmbedViewExternal {
    /// The URI of the external content.
    #[serde(rename = "uri")]
    pub uri: String,

    /// The title of the external content.
    #[serde(rename = "title")]
    pub title: String,

    /// A description of the external content.
    #[serde(rename = "description")]
    pub description: String,

    /// A URI to a thumbnail image representing the external content.
    #[serde(rename = "thumb", skip_serializing_if = "Option::is_none")]
    pub thumb: Option<String>
}
