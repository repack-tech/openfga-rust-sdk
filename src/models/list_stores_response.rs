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
pub struct ListStoresResponse {
    #[serde(rename = "stores", skip_serializing_if = "Option::is_none")]
    pub stores: Option<Vec<crate::models::Store>>,
    /// The continuation token will be empty if there are no more stores.
    #[serde(rename = "continuation_token", skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}

impl ListStoresResponse {
    pub fn new() -> ListStoresResponse {
        ListStoresResponse {
            stores: None,
            continuation_token: None,
        }
    }
}

