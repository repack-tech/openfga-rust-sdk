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

/// UsersetTree : A UsersetTree contains the result of an Expansion.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UsersetTree {
    #[serde(rename = "root", skip_serializing_if = "Option::is_none")]
    pub root: Option<Box<crate::models::Node>>,
}

impl UsersetTree {
    /// A UsersetTree contains the result of an Expansion.
    pub fn new() -> UsersetTree {
        UsersetTree {
            root: None,
        }
    }
}


