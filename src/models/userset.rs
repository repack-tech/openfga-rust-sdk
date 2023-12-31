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
pub struct Userset {
    /// A DirectUserset is a sentinel message for referencing the direct members specified by an object/relation mapping.
    #[serde(rename = "this", skip_serializing_if = "Option::is_none")]
    pub this: Option<serde_json::Value>,
    #[serde(rename = "computedUserset", skip_serializing_if = "Option::is_none")]
    pub computed_userset: Option<Box<crate::models::ObjectRelation>>,
    #[serde(rename = "tupleToUserset", skip_serializing_if = "Option::is_none")]
    pub tuple_to_userset: Option<Box<crate::models::TupleToUserset>>,
    #[serde(rename = "union", skip_serializing_if = "Option::is_none")]
    pub union: Option<Box<crate::models::Usersets>>,
    #[serde(rename = "intersection", skip_serializing_if = "Option::is_none")]
    pub intersection: Option<Box<crate::models::Usersets>>,
    #[serde(rename = "difference", skip_serializing_if = "Option::is_none")]
    pub difference: Option<Box<crate::models::Difference>>,
}

impl Userset {
    pub fn new() -> Userset {
        Userset {
            this: None,
            computed_userset: None,
            tuple_to_userset: None,
            union: None,
            intersection: None,
            difference: None,
        }
    }
}


