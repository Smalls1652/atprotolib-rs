use serde::{Deserialize, Serialize};

use super::{LabelerView, LabelerViewDetailed};

/*
    app.bsky.labeler.getServices
*/

/*    Type: response
    Id: app.bsky.labeler.getServices#response
    Kind: object
    
    Properties:
    - views: union[] (JsonProperty: views) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetServicesResponse {
    views: Vec<LabelerViewDetailed>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GetServicesResponseViews {
    LabelerView(LabelerView),
    LabelerViewDetailed(LabelerViewDetailed)
}
