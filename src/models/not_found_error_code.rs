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


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NotFoundErrorCode {
    #[serde(rename = "no_not_found_error")]
    NoNotFoundError,
    #[serde(rename = "undefined_endpoint")]
    UndefinedEndpoint,
    #[serde(rename = "store_id_not_found")]
    StoreIdNotFound,
    #[serde(rename = "unimplemented")]
    Unimplemented,

}

impl ToString for NotFoundErrorCode {
    fn to_string(&self) -> String {
        match self {
            Self::NoNotFoundError => String::from("no_not_found_error"),
            Self::UndefinedEndpoint => String::from("undefined_endpoint"),
            Self::StoreIdNotFound => String::from("store_id_not_found"),
            Self::Unimplemented => String::from("unimplemented"),
        }
    }
}

impl Default for NotFoundErrorCode {
    fn default() -> NotFoundErrorCode {
        Self::NoNotFoundError
    }
}




