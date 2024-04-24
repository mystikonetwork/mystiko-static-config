use std::hash::Hash;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::Validate;

#[derive(TypedBuilder, Validate, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct RawSequencerConfig {
    pub host: String,
    #[serde(default)]
    #[builder(default)]
    pub port: Option<u16>,
    #[serde(default)]
    #[builder(default)]
    pub is_ssl: bool,
}
