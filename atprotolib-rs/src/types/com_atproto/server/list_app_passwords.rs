use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/*
    com.atproto.server.listAppPasswords
*/

/// Represents a list app passwords response.
///
/// [`com.atproto.server.listAppPasswords#responses`](https://docs.bsky.app/docs/api/com-atproto-server-list-app-passwords#responses)
#[derive(Serialize, Deserialize, Debug)]
pub struct ListAppPasswordsResponse {
    #[serde(rename = "passwords")]
    pub passwords: Vec<ListAppPasswordsResponseAppPassword>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListAppPasswordsResponseAppPassword {
    /// A short name for the App Password, to help distinguish them.
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    #[serde(rename = "privileged", default)]
    pub privileged: bool
}
