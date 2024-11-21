use serde::{Deserialize, Serialize};

/*    Type: label
    Id: com.atproto.label.defs#label
    Kind: object
    
    Properties:
    - ver: integer  (JsonProperty: ver) [Optional]
    - src: string (JsonProperty: src) [Required]
    - uri: string (JsonProperty: uri) [Required]
    - cid: string (JsonProperty: cid) [Optional]
    - val: string (JsonProperty: val) [Required]
    - neg: boolean  (JsonProperty: neg) [Optional]
    - cts: datetime (JsonProperty: cts) [Required]
    - exp: datetime (JsonProperty: exp) [Optional]
    - sig: bytes  (JsonProperty: sig) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.label.defs#label")]
pub struct Label {
    #[serde(rename = "ver")]
    pub ver: i32,
    #[serde(rename = "src")]
    pub src: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(rename = "val")]
    pub val: String,
    #[serde(rename = "neg")]
    pub neg: bool,
    #[serde(rename = "cts")]
    pub cts: String,
    #[serde(rename = "exp", skip_serializing_if = "Option::is_none")]
    pub exp: Option<String>,
    #[serde(rename = "sig", skip_serializing_if = "Option::is_none")]
    pub sig: Option<Vec<u8>>
}

/*    Type: selfLabels
    Id: com.atproto.label.defs#selfLabels
    Kind: object
    
    Properties:
    - values: #selfLabel[] (JsonProperty: values) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.label.defs#selfLabels")]
pub struct SelfLabels {
    #[serde(rename = "values")]
    pub values: Vec<SelfLabel>
}

/*    Type: selfLabel
    Id: com.atproto.label.defs#selfLabel
    Kind: object
    
    Properties:
    - val: string (JsonProperty: val) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.label.defs#selfLabel")]
pub struct SelfLabel {
    #[serde(rename = "val")]
    pub val: String
}

/*    Type: labelValueDefinition
    Id: com.atproto.label.defs#labelValueDefinition
    Kind: object
    
    Properties:
    - identifier: string (JsonProperty: identifier) [Required]
    - severity: string (JsonProperty: severity) [Required]
    - blurs: string (JsonProperty: blurs) [Required]
    - default_setting: string (JsonProperty: defaultSetting) [Optional]
    - adult_only: boolean  (JsonProperty: adultOnly) [Optional]
    - locales: #labelValueDefinitionStrings[] (JsonProperty: locales) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.label.defs#labelValueDefinition")]
pub struct LabelValueDefinition {
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "severity")]
    pub severity: String,
    #[serde(rename = "blurs")]
    pub blurs: String,
    #[serde(rename = "defaultSetting", skip_serializing_if = "Option::is_none")]
    pub default_setting: Option<String>,
    #[serde(rename = "adultOnly")]
    pub adult_only: bool,
    #[serde(rename = "locales")]
    pub locales: Vec<LabelValueDefinitionStrings>
}

/*    Type: labelValueDefinitionStrings
    Id: com.atproto.label.defs#labelValueDefinitionStrings
    Kind: object
    
    Properties:
    - lang: string (JsonProperty: lang) [Required]
    - name: string (JsonProperty: name) [Required]
    - description: string (JsonProperty: description) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.label.defs#labelValueDefinitionStrings")]
pub struct LabelValueDefinitionStrings {
    #[serde(rename = "lang")]
    pub lang: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String
}
