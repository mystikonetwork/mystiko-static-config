use crate::{
    decimal_to_number, AssetConfig, CircuitConfig, PoolContractConfig, RawDepositContractConfig,
};
use anyhow::{Error, Result};
use mystiko_types::{AssetType, BridgeType, ContractType};
use num_bigint::BigUint;
use num_traits::{NumCast, Zero};
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;
use typed_builder::TypedBuilder;
use validator::Validate;

#[derive(Clone, Debug, TypedBuilder)]
pub struct DepositContractConfig {
    raw: Arc<RawDepositContractConfig>,
    bridge_fee_asset_config: Arc<AssetConfig>,
    executor_fee_asset_config: Arc<AssetConfig>,
    pool_contract_config: Arc<PoolContractConfig>,
}

impl DepositContractConfig {
    pub fn new(
        raw: Arc<RawDepositContractConfig>,
        bridge_fee_asset_config: Arc<AssetConfig>,
        executor_fee_asset_config: Arc<AssetConfig>,
        pool_contract_config: Arc<PoolContractConfig>,
    ) -> Self {
        DepositContractConfig {
            raw,
            bridge_fee_asset_config,
            executor_fee_asset_config,
            pool_contract_config,
        }
    }

    pub fn version(&self) -> u32 {
        self.raw.version
    }

    pub fn name(&self) -> &str {
        &self.raw.name
    }

    pub fn address(&self) -> &str {
        &self.raw.address
    }

    pub fn contract_type(&self) -> &ContractType {
        &self.raw.contract_type
    }

    pub fn start_block(&self) -> u64 {
        self.raw.start_block
    }

    pub fn event_filter_size(&self) -> &Option<u64> {
        &self.raw.event_filter_size
    }

    pub fn bridge_type(&self) -> &BridgeType {
        &self.raw.bridge_type
    }

    pub fn pool_contract_address(&self) -> &str {
        &self.raw.pool_address
    }

    pub fn pool_contract(&self) -> &PoolContractConfig {
        &self.pool_contract_config
    }

    pub fn disabled(&self) -> bool {
        self.disabled_at().is_some()
    }

    pub fn disabled_at(&self) -> &Option<u64> {
        &self.raw.disabled_at
    }

    pub fn min_amount(&self) -> Result<BigUint> {
        Ok(BigUint::from_str(&self.raw.min_amount)?)
    }

    pub fn min_amount_number<T>(&self) -> Result<T>
    where
        T: NumCast + Zero,
    {
        let asset_decimals = self.asset().asset_decimals();
        decimal_to_number::<T, String>(&self.raw.min_amount, Some(asset_decimals))
    }

    pub fn max_amount(&self) -> Result<BigUint> {
        Ok(BigUint::from_str(&self.raw.max_amount)?)
    }

    pub fn max_amount_number<T>(&self) -> Result<T>
    where
        T: NumCast + Zero,
    {
        let asset_decimals = self.asset().asset_decimals();
        decimal_to_number::<T, String>(&self.raw.max_amount, Some(asset_decimals))
    }

    pub fn min_bridge_fee(&self) -> Result<BigUint> {
        Ok(BigUint::from_str(&self.raw.min_bridge_fee)?)
    }

    pub fn min_bridge_fee_number<T>(&self) -> Result<T>
    where
        T: NumCast + Zero,
    {
        let asset_decimals = self.bridge_fee_asset().asset_decimals();
        decimal_to_number::<T, String>(&self.raw.min_bridge_fee, Some(asset_decimals))
    }

    pub fn min_executor_fee(&self) -> Result<BigUint> {
        Ok(BigUint::from_str(&self.raw.min_executor_fee)?)
    }

    pub fn min_executor_fee_number<T>(&self) -> Result<T>
    where
        T: NumCast + Zero,
    {
        let asset_decimals = self.executor_fee_asset().asset_decimals();
        decimal_to_number::<T, String>(&self.raw.min_executor_fee, Some(asset_decimals))
    }

    pub fn min_rollup_fee(&self) -> Result<BigUint> {
        self.pool_contract().min_rollup_fee()
    }

    pub fn min_rollup_fee_number<T>(&self) -> Result<T>
    where
        T: NumCast + Zero,
    {
        self.pool_contract().min_rollup_fee_number()
    }

    pub fn service_fee(&self) -> u32 {
        self.raw.service_fee
    }

    pub fn service_fee_divider(&self) -> u32 {
        self.raw.service_fee_divider
    }

    pub fn asset(&self) -> &AssetConfig {
        self.pool_contract().asset()
    }

    pub fn asset_address(&self) -> Option<&str> {
        self.pool_contract().asset_address()
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
        self.asset().recommended_amounts_number()
    }

    pub fn bridge_fee_asset(&self) -> &AssetConfig {
        &self.bridge_fee_asset_config
    }

    pub fn bridge_fee_asset_address(&self) -> Option<&str> {
        self.raw.bridge_fee_asset_address.as_deref()
    }

    pub fn executor_fee_asset(&self) -> &AssetConfig {
        &self.executor_fee_asset_config
    }

    pub fn executor_fee_asset_address(&self) -> Option<&str> {
        self.raw.executor_fee_asset_address.as_deref()
    }

    pub fn circuits(&self) -> Vec<&CircuitConfig> {
        self.pool_contract().circuits()
    }

    pub fn peer_chain_id(&self) -> &Option<u64> {
        &self.raw.peer_chain_id
    }

    pub fn peer_contract_address(&self) -> Option<&str> {
        self.raw.peer_contract_address.as_deref()
    }

    pub fn validate(&self) -> Result<()> {
        self.raw.validate()?;
        if self.min_amount()? > self.max_amount()? {
            return Err(Error::msg(format!(
                "deposit contract {} min_amount is greater than max_amount",
                self.address()
            )));
        }
        if let Some(bridge_fee_asset_address) = self.bridge_fee_asset_address() {
            if bridge_fee_asset_address != self.bridge_fee_asset().asset_address() {
                return Err(Error::msg(format!(
                    "deposit contract {} configured bridge_fee_asset_address {} \
                    is different with the given bridge_fee_asset_config asset_address {}",
                    self.address(),
                    bridge_fee_asset_address,
                    self.bridge_fee_asset().asset_address()
                )));
            }
        }
        if let Some(executor_fee_asset_address) = self.executor_fee_asset_address() {
            if executor_fee_asset_address != self.executor_fee_asset().asset_address() {
                return Err(Error::msg(format!(
                    "deposit contract {} configured executor_fee_asset_address {} \
                    is different with the given executor_fee_asset_config asset_address {}",
                    self.address(),
                    executor_fee_asset_address,
                    self.executor_fee_asset().asset_address()
                )));
            }
        }
        if self.bridge_type() == &BridgeType::Loop {
            if self.peer_contract_address().is_some() || self.peer_chain_id().is_some() {
                return Err(Error::msg(format!(
                    "deposit contract {} peer_contract_address and peer_contract_address \
                    should be None when bridge_type is LOOP",
                    self.address()
                )));
            }
        } else if self.peer_contract_address().is_none() || self.peer_chain_id().is_none() {
            return Err(Error::msg(format!(
                "deposit contract {} peer_contract_address and peer_contract_address \
                    should NOT be None when bridge_type is NOT LOOP",
                self.address()
            )));
        }
        if self.pool_contract_address() != self.pool_contract_config.address() {
            return Err(Error::msg(format!(
                "mismatched pool contract address {} vs {} for deposit contract config at {}",
                self.pool_contract_address(),
                self.pool_contract_config.address(),
                self.address()
            )));
        }
        if self.bridge_type() != self.pool_contract_config.bridge_type() {
            return Err(Error::msg(format!(
                "mismatched pool contract bridge_type {:?} vs {:?} \
                for deposit contract config at {}",
                self.bridge_type(),
                self.pool_contract_config.bridge_type(),
                self.address()
            )));
        }
        if let Some(disabled_at) = self.disabled_at() {
            if *disabled_at < self.start_block() {
                return Err(Error::msg(format!(
                    "deposit contract {} disabled_at {} is less than start_block {}",
                    self.address(),
                    disabled_at,
                    self.start_block()
                )));
            }
        } else if self.pool_contract().disabled() {
            return Err(Error::msg(format!(
                "deposit contract {} pool contract {} is disabled",
                self.address(),
                self.pool_contract().address()
            )));
        }
        Ok(())
    }
}
