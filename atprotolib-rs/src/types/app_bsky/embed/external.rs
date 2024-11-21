use serde::{Deserialize, Serialize};

/*
    app.bsky.embed.external
*/

/*    Type: external
    Id: app.bsky.embed.external#external
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - title: string (JsonProperty: title) [Required]
    - description: string (JsonProperty: description) [Required]
    - thumb: blob  (JsonProperty: thumb) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.embed.external")]
pub struct ExternalEmbed {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "thumb", skip_serializing_if = "Option::is_none")]
    pub thumb: Option<Vec<u8>>
}

/*    Type: view
    Id: app.bsky.embed.external#view
    Kind: object

    Properties:
    - external: #viewExternal (JsonProperty: external) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.embed.external#view")]
pub struct ExternalEmbedView {
    #[serde(rename = "external")]
    pub external: ExternalEmbed
}

/*    Type: viewExternal
    Id: app.bsky.embed.external#viewExternal
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - title: string (JsonProperty: title) [Required]
    - description: string (JsonProperty: description) [Required]
    - thumb: string (JsonProperty: thumb) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.embed.external#viewExternal")]
pub struct ExternalEmbedViewExternal {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "thumb", skip_serializing_if = "Option::is_none")]
    pub thumb: Option<String>
}
