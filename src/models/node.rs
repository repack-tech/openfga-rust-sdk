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
pub struct Node {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "leaf", skip_serializing_if = "Option::is_none")]
    pub leaf: Option<Box<crate::models::Leaf>>,
    #[serde(rename = "difference", skip_serializing_if = "Option::is_none")]
    pub difference: Option<Box<crate::models::UsersetTreePeriodDifference>>,
    #[serde(rename = "union", skip_serializing_if = "Option::is_none")]
    pub union: Option<Box<crate::models::Nodes>>,
    #[serde(rename = "intersection", skip_serializing_if = "Option::is_none")]
    pub intersection: Option<Box<crate::models::Nodes>>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            name: None,
            leaf: None,
            difference: None,
            union: None,
            intersection: None,
        }
    }
}


