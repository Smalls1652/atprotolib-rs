use serde::{Deserialize, Serialize};

/*
    com.atproto.server.describeServer
*/

/// Represents a server description response.
///
/// [`com.atproto.server.describeServer#responses`](https://docs.bsky.app/docs/api/com-atproto-server-describe-server#responses)
#[derive(Serialize, Deserialize, Debug)]
pub struct DescribeServerResponse {
    #[serde(rename = "inviteCodeRequired")]
    pub invite_code_required: bool,

    #[serde(rename = "phoneVerificationRequired")]
    pub phone_verification_required: bool,

    #[serde(rename = "availableUserDomains")]
    pub available_user_domains: Vec<String>,

    #[serde(rename = "links")]
    pub links: DescribeServerResponseLinks,

    #[serde(rename = "contact")]
    pub contact: DescribeServerResponseContact
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DescribeServerResponseLinks {
    #[serde(rename = "privacyPolicy", skip_serializing_if = "Option::is_none")]
    pub privacy_policy: Option<String>,

    #[serde(rename = "termsOfService", skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DescribeServerResponseContact {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>
}
