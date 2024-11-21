use serde::{Deserialize, Serialize};

/*
    com.atproto.server.reserveSigningKey
*/

/// Represents a signing key reservation request.
///
/// [`com.atproto.server.reserveSigningKey#request`](https://docs.bsky.app/docs/api/com-atproto-server-reserve-signing-key#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct ReserveSigningKeyRequest {
    #[serde(rename = "did")]
    pub did: String
}

/// Represents a signing key reservation response.
///
/// [`com.atproto.server.reserveSigningKey#responses`](https://docs.bsky.app/docs/api/com-atproto-server-reserve-signing-key#responses)
#[derive(Serialize, Deserialize, Debug)]
pub struct ReserveSigningKeyResponse {
    #[serde(rename = "signingKey")]
    pub signing_key: String
}
