use serde::{Deserialize, Serialize};

use super::InviteCode;

/*
    com.atproto.server.getAccountInviteCodes
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAccountInviteCodesRequest {
    #[serde(rename = "codes")]
    pub codes: Vec<InviteCode>
}
