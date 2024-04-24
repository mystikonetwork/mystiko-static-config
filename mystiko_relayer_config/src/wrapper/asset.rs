use crate::raw::asset::RawAssetConfig;
use mystiko_types::AssetType;
use rust_decimal::Decimal;
use std::sync::Arc;
use validator::Validate;

#[derive(Clone, Debug, PartialEq)]
pub struct AssetConfig {
    raw: Arc<RawAssetConfig>,
}

impl AssetConfig {
    pub fn new(raw: Arc<RawAssetConfig>) -> Self {
        AssetConfig { raw }
    }

    pub fn asset_type(&self) -> &AssetType {
        &self.raw.asset_type
    }

    pub fn asset_symbol(&self) -> &str {
        &self.raw.asset_symbol
    }

    pub fn asset_decimals(&self) -> u32 {
        self.raw.asset_decimals
    }

    pub fn relayer_fee_of_ten_thousandth(&self) -> u32 {
        self.raw.relayer_fee_of_ten_thousandth
    }

    pub fn minimum_gas_fee(&self) -> &Decimal {
        &self.raw.minimum_gas_fee
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        Ok(self.raw.validate()?)
    }
}
