use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/*    Type: inviteCode
    Id: com.atproto.server.defs#inviteCode
    Kind: object
    
    Properties:
    - code: string (JsonProperty: code) [Required]
    - available: integer  (JsonProperty: available) [Required]
    - disabled: boolean  (JsonProperty: disabled) [Required]
    - for_account: string (JsonProperty: forAccount) [Required]
    - created_by: string (JsonProperty: createdBy) [Required]
    - created_at: datetime (JsonProperty: createdAt) [Required]
    - uses: #inviteCodeUse[] (JsonProperty: uses) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.server.defs#inviteCode")]
pub struct InviteCode {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "available")]
    pub available: i32,
    #[serde(rename = "disabled")]
    pub disabled: bool,
    #[serde(rename = "forAccount")]
    pub for_account: String,
    #[serde(rename = "createdBy")]
    pub created_by: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "uses")]
    pub uses: Vec<InviteCodeUse>
}

/*    Type: inviteCodeUse
    Id: com.atproto.server.defs#inviteCodeUse
    Kind: object
    
    Properties:
    - used_by: string (JsonProperty: usedBy) [Required]
    - used_at: datetime (JsonProperty: usedAt) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.server.defs#inviteCodeUse")]
pub struct InviteCodeUse {
    #[serde(rename = "usedBy")]
    pub used_by: String,
    #[serde(rename = "usedAt")]
    pub used_at: DateTime<Utc>
}
