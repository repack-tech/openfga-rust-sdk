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
pub struct AuthorizationModel {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "schema_version")]
    pub schema_version: String,
    #[serde(rename = "type_definitions", skip_serializing_if = "Option::is_none")]
    pub type_definitions: Option<Vec<crate::models::TypeDefinition>>,
}

impl AuthorizationModel {
    pub fn new(schema_version: String) -> AuthorizationModel {
        AuthorizationModel {
            id: None,
            schema_version,
            type_definitions: None,
        }
    }
}

