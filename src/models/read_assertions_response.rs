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
pub struct ReadAssertionsResponse {
    #[serde(rename = "authorization_model_id", skip_serializing_if = "Option::is_none")]
    pub authorization_model_id: Option<String>,
    #[serde(rename = "assertions", skip_serializing_if = "Option::is_none")]
    pub assertions: Option<Vec<crate::models::Assertion>>,
}

impl ReadAssertionsResponse {
    pub fn new() -> ReadAssertionsResponse {
        ReadAssertionsResponse {
            authorization_model_id: None,
            assertions: None,
        }
    }
}


