/**
 * rust SDK for OpenFGA
 *
 * API version: 0.1
 * Website: https://openfga.dev
 * Documentation: https://openfga.dev/docs
 * Support: https://discord.gg/8naAwJfWN6
 * License: [Apache-2.0](https://github.com/openfga/rust-sdk/blob/main/LICENSE)
 *
 * NOTE: This file was auto generated by OpenAPI Generator (https://openapi-generator.tech). DO NOT EDIT.
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListObjectsRequest {
    #[serde(rename = "authorization_model_id", skip_serializing_if = "Option::is_none")]
    pub authorization_model_id: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "relation")]
    pub relation: String,
    #[serde(rename = "user")]
    pub user: String,
    #[serde(rename = "contextual_tuples", skip_serializing_if = "Option::is_none")]
    pub contextual_tuples: Option<Box<crate::models::ContextualTupleKeys>>,
}

impl ListObjectsRequest {
    pub fn new(r#type: String, relation: String, user: String) -> ListObjectsRequest {
        ListObjectsRequest {
            authorization_model_id: None,
            r#type,
            relation,
            user,
            contextual_tuples: None,
        }
    }
}


