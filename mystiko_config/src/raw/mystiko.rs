use crate::RawChainConfig;
use crate::RawCircuitConfig;
use crate::RawPackerConfig;
use crate::RawScreeningConfig;
use crate::{RawBridgeConfig, RawSequencerConfig};
use mystiko_validator::validate::{
    array_unique, is_git_revision, is_sem_ver, string_vec_each_not_empty, validate_nested_vec,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use validator::Validate;

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RawMystikoConfig {
    #[validate(custom = "is_sem_ver")]
    pub version: String,

    #[validate(custom = "is_git_revision")]
    #[serde(default)]
    pub git_revision: Option<String>,

    #[validate(
        custom(function = "array_unique"),
        custom(function = "validate_nested_vec")
    )]
    #[serde(default)]
    pub chains: Vec<Arc<RawChainConfig>>,

    #[validate(
        custom(function = "array_unique"),
        custom(function = "validate_nested_vec")
    )]
    #[serde(default)]
    pub bridges: Vec<Arc<RawBridgeConfig>>,

    #[validate(
        custom(function = "array_unique"),
        custom(function = "validate_nested_vec")
    )]
    #[serde(default)]
    pub circuits: Vec<Arc<RawCircuitConfig>>,

    #[validate]
    #[serde(default)]
    pub sequencer: Option<Arc<RawSequencerConfig>>,

    #[validate]
    #[serde(default)]
    pub packer: Option<Arc<RawPackerConfig>>,

    #[validate]
    #[serde(default)]
    pub screening: Option<Arc<RawScreeningConfig>>,

    #[validate(
        custom(function = "array_unique"),
        custom(function = "string_vec_each_not_empty")
    )]
    #[serde(default)]
    pub country_blacklist: Vec<String>,
}
