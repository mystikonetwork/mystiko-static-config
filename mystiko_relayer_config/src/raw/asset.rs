use mystiko_types::AssetType;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::Validate;

#[derive(TypedBuilder, Validate, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RawAssetConfig {
    pub asset_type: AssetType,

    #[validate(length(min = 1))]
    pub asset_symbol: String,

    #[validate(range(min = 1))]
    #[serde(default = "default_asset_decimals")]
    #[builder(default = default_asset_decimals())]
    pub asset_decimals: u32,

    #[validate(range(min = 0))]
    #[serde(default = "default_relayer_fee_of_ten_thousandth")]
    #[builder(default = default_relayer_fee_of_ten_thousandth())]
    pub relayer_fee_of_ten_thousandth: u32,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    #[serde(default = "default_minimum_gas_fee")]
    #[builder(default = default_minimum_gas_fee())]
    pub minimum_gas_fee: Decimal,
}

fn default_minimum_gas_fee() -> Decimal {
    Decimal::ZERO
}

fn default_asset_decimals() -> u32 {
    18
}

fn default_relayer_fee_of_ten_thousandth() -> u32 {
    25
}
