use crate::raw::chain::RawChainConfig;
use mystiko_validator::validate::{is_git_revision, is_sem_ver};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use typed_builder::TypedBuilder;
use validator::Validate;

#[derive(TypedBuilder, Validate, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RawRelayerConfig {
    #[validate(custom = "is_sem_ver")]
    pub version: String,

    #[validate(custom = "is_git_revision")]
    #[serde(default)]
    pub git_revision: Option<String>,

    #[validate]
    #[serde(default)]
    pub chains: Vec<Arc<RawChainConfig>>,
}
