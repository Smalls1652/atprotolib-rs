use serde::{Deserialize, Serialize};

use super::{NotFoundActor, Relationship};

/*
    app.bsky.graph.getRelationships
*/

/*    Type: response
    Id: app.bsky.graph.getRelationships#response
    Kind: object
    
    Properties:
    - actor: string (JsonProperty: actor) [Optional]
    - relationships: union[] (JsonProperty: relationships) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetRelationshipsResponse {
    actor: Option<String>,
    relationships: Vec<GetRelationshipsResponseRelationships>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GetRelationshipsResponseRelationships {
    Relationship(Relationship),
    NotFoundActor(NotFoundActor)
}
