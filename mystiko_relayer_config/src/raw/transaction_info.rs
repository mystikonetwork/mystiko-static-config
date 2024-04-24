use crate::raw::gas_cost::RawGasCostConfig;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use typed_builder::TypedBuilder;
use validator::Validate;

#[derive(TypedBuilder, Validate, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RawTransactionInfoConfig {
    #[validate]
    pub main_gas_cost: Arc<RawGasCostConfig>,

    #[validate]
    pub erc20_gas_cost: Arc<RawGasCostConfig>,
}
