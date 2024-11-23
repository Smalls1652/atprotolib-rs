use serde::{Deserialize, Serialize};

/*
    com.atproto.sync.requestCrawl
*/

/*    Type: request
    Id: com.atproto.sync.requestCrawl#request
    Kind: object

    Properties:
    - hostname: string (JsonProperty: hostname) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCrawlRequest {
    #[serde(rename = "hostname")]
    pub hostname: String
}
