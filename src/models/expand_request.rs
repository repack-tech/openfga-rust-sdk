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
pub struct ExpandRequest {
    #[serde(rename = "tuple_key")]
    pub tuple_key: Box<crate::models::TupleKey>,
    #[serde(rename = "authorization_model_id", skip_serializing_if = "Option::is_none")]
    pub authorization_model_id: Option<String>,
}

impl ExpandRequest {
    pub fn new(tuple_key: crate::models::TupleKey) -> ExpandRequest {
        ExpandRequest {
            tuple_key: Box::new(tuple_key),
            authorization_model_id: None,
        }
    }
}


