use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/*
    com.atproto.server.createAppPassword
*/

/// Represents an app password creation request.
///
/// [`com.atproto.server.createAppPassword#request`](https://docs.bsky.app/docs/api/com-atproto-server-create-app-password#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAppPasswordRequest {
    /// A short name for the App Password, to help distinguish them.
    #[serde(rename = "name")]
    pub name: String,

    /// If an app password has 'privileged' access to possibly sensitive account
    /// state. Meant for use with trusted clients.
    #[serde(rename = "privileged")]
    pub privileged: bool
}

/// Represents an app password creation response.
///
/// [`com.atproto.server.createAppPassword#responses`](https://docs.bsky.app/docs/api/com-atproto-server-create-app-password#responses)
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAppPasswordResponse {
    /// A short name for the App Password, to help distinguish them.
    #[serde(rename = "name")]
    pub name: String,

    /// The app password.
    #[serde(rename = "password")]
    pub password: String,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    #[serde(rename = "privileged")]
    pub privileged: bool
}
