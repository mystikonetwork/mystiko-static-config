mod deposit;
mod pool;

pub use deposit::*;
pub use pool::*;

use crate::AssetConfig;
use anyhow::Result;
use mystiko_types::{AssetType, BridgeType, ContractType};
use num_bigint::BigUint;
use num_traits::{NumCast, Zero};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum ContractConfig {
    Deposit(Arc<DepositContractConfig>),
    Pool(Arc<PoolContractConfig>),
}

impl ContractConfig {
    pub fn version(&self) -> u32 {
        match self {
            ContractConfig::Deposit(config) => config.version(),
            ContractConfig::Pool(config) => config.version(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ContractConfig::Deposit(config) => config.name(),
            ContractConfig::Pool(config) => config.name(),
        }
    }

    pub fn address(&self) -> &str {
        match self {
            ContractConfig::Deposit(config) => config.address(),
            ContractConfig::Pool(config) => config.address(),
        }
    }

    pub fn contract_type(&self) -> &ContractType {
        match self {
            ContractConfig::Deposit(config) => config.contract_type(),
            ContractConfig::Pool(config) => config.contract_type(),
        }
    }

    pub fn bridge_type(&self) -> &BridgeType {
        match self {
            ContractConfig::Deposit(config) => config.bridge_type(),
            ContractConfig::Pool(config) => config.bridge_type(),
        }
    }

    pub fn disabled(&self) -> bool {
        match self {
            ContractConfig::Deposit(config) => config.disabled(),
            ContractConfig::Pool(config) => config.disabled(),
        }
    }

    pub fn disabled_at(&self) -> &Option<u64> {
        match self {
            ContractConfig::Deposit(config) => config.disabled_at(),
            ContractConfig::Pool(config) => config.disabled_at(),
        }
    }

    pub fn start_block(&self) -> u64 {
        match self {
            ContractConfig::Deposit(config) => config.start_block(),
            ContractConfig::Pool(config) => config.start_block(),
        }
    }

    pub fn event_filter_size(&self) -> &Option<u64> {
        match self {
            ContractConfig::Deposit(config) => config.event_filter_size(),
            ContractConfig::Pool(config) => config.event_filter_size(),
        }
    }

    pub fn min_rollup_fee(&self) -> Result<BigUint> {
        match self {
            ContractConfig::Deposit(config) => config.min_rollup_fee(),
            ContractConfig::Pool(config) => config.min_rollup_fee(),
        }
    }

    pub fn min_rollup_fee_number<T>(&self) -> Result<T>
    where
        T: NumCast + Zero,
    {
        match self {
            ContractConfig::Deposit(config) => config.min_rollup_fee_number::<T>(),
            ContractConfig::Pool(config) => config.min_rollup_fee_number::<T>(),
        }
    }

    pub fn asset(&self) -> &AssetConfig {
        match self {
            ContractConfig::Deposit(config) => config.asset(),
            ContractConfig::Pool(config) => config.asset(),
        }
    }

    pub fn asset_address(&self) -> Option<&str> {
        match self {
            ContractConfig::Deposit(config) => config.asset_address(),
            ContractConfig::Pool(config) => config.asset_address(),
        }
    }

    pub fn asset_type(&self) -> &AssetType {
        self.asset().asset_type()
    }

    pub fn asset_symbol(&self) -> &str {
        self.asset().asset_symbol()
    }

    pub fn asset_decimals(&self) -> u32 {
        self.asset().asset_decimals()
    }

    pub fn recommended_amounts(&self) -> Result<Vec<BigUint>> {
        self.asset().recommended_amounts()
    }

    pub fn recommended_amounts_number<T>(&self) -> Result<Vec<T>>
    where
        T: NumCast + Zero,
    {
        self.asset().recommended_amounts_number::<T>()
    }

    pub fn validate(&self) -> Result<()> {
        match self {
            ContractConfig::Deposit(config) => config.validate(),
            ContractConfig::Pool(config) => config.validate(),
        }
    }
}
