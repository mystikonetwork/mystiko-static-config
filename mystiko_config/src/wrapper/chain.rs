use crate::{
    AssetConfig, CircuitConfig, ContractConfig, DepositContractConfig, PoolContractConfig,
    ProviderConfig, RawAssetConfig, RawChainConfig, RawDepositContractConfig,
    RawPoolContractConfig, EXPLORER_TX_PLACEHOLDER, MAIN_ASSET_ADDRESS,
};
use anyhow::{Error, Result};
use mystiko_types::{AssetType, BridgeType, CircuitType, ProviderType, TransactionType};
use num_bigint::BigUint;
use num_traits::{NumCast, Zero};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use validator::Validate;

#[derive(Clone, Debug)]
pub struct ChainConfig {
    raw: Arc<RawChainConfig>,
    main_asset_config: Arc<AssetConfig>,
    asset_configs: Vec<Arc<AssetConfig>>,
    pool_contract_configs: Vec<Arc<PoolContractConfig>>,
    deposit_contract_configs: Vec<Arc<DepositContractConfig>>,
    provider_configs: Vec<Arc<ProviderConfig>>,
    packer_sorted_granularities: Vec<u64>,
}

impl ChainConfig {
    pub fn new(
        raw: Arc<RawChainConfig>,
        default_circuit_configs: &HashMap<CircuitType, Arc<CircuitConfig>>,
        circuit_configs_by_name: &HashMap<String, Arc<CircuitConfig>>,
    ) -> Result<Self> {
        let main_asset_config = Arc::new(AssetConfig::new(Arc::new(RawAssetConfig {
            asset_type: AssetType::Main,
            asset_symbol: raw.asset_symbol.to_string(),
            asset_symbol_alias: raw.asset_symbol_alias.clone(),
            asset_decimals: raw.asset_decimals,
            asset_address: MAIN_ASSET_ADDRESS.to_string(),
            recommended_amounts: raw.recommended_amounts.clone(),
        })));
        let asset_configs: Vec<Arc<AssetConfig>> = raw
            .assets
            .iter()
            .map(|r| Arc::new(AssetConfig::new(r.clone())))
            .collect();
        let provider_configs = raw
            .providers
            .iter()
            .map(|r| Arc::new(ProviderConfig::new(r.clone())))
            .collect();
        let pool_contract_configs = initialize_pool_contracts(
            &raw.pool_contracts,
            &main_asset_config,
            &asset_configs,
            default_circuit_configs,
            circuit_configs_by_name,
        )?;
        let deposit_contract_configs = initialize_deposit_contracts(
            &raw.deposit_contracts,
            &pool_contract_configs,
            &main_asset_config,
            &asset_configs,
        )?;
        let mut packer_sorted_granularities = raw.packer_granularities.clone();
        packer_sorted_granularities.sort();
        Ok(ChainConfig {
            raw,
            main_asset_config,
            asset_configs,
            pool_contract_configs,
            deposit_contract_configs,
            provider_configs,
            packer_sorted_granularities,
        })
    }

    pub fn chain_id(&self) -> u64 {
        self.raw.chain_id
    }

    pub fn name(&self) -> &str {
        &self.raw.name
    }

    pub fn main_asset(&self) -> &AssetConfig {
        &self.main_asset_config
    }

    pub fn asset_symbol(&self) -> &str {
        &self.raw.asset_symbol
    }

    pub fn asset_decimals(&self) -> u32 {
        self.raw.asset_decimals
    }

    pub fn recommended_amounts(&self) -> Result<Vec<BigUint>> {
        self.main_asset().recommended_amounts()
    }

    pub fn recommended_amounts_number<T>(&self) -> Result<Vec<T>>
    where
        T: NumCast + Zero,
    {
        self.main_asset().recommended_amounts_number()
    }

    pub fn explorer_url(&self) -> &str {
        &self.raw.explorer_url
    }

    pub fn explorer_api_url(&self) -> &str {
        &self.raw.explorer_api_url
    }

    pub fn explorer_prefix(&self) -> &str {
        &self.raw.explorer_prefix
    }

    pub fn providers(&self) -> Vec<&ProviderConfig> {
        self.provider_configs.iter().map(|c| c.as_ref()).collect()
    }

    pub fn provider_type(&self) -> &ProviderType {
        &self.raw.provider_type
    }

    pub fn provider_quorum_percentage(&self) -> u8 {
        self.raw.provider_quorum_percentage
    }

    pub fn signer_endpoint(&self) -> &str {
        &self.raw.signer_endpoint
    }

    pub fn transaction_type(&self) -> &TransactionType {
        &self.raw.transaction_type
    }

    pub fn event_delay_blocks(&self) -> u64 {
        self.raw.event_delay_blocks
    }

    pub fn event_filter_size(&self) -> u64 {
        self.raw.event_filter_size
    }

    pub fn sequencer_fetch_size(&self) -> u64 {
        self.raw.sequencer_fetch_size
    }

    pub fn safe_confirmations(&self) -> Option<u64> {
        self.raw.safe_confirmations
    }

    pub fn granularities(&self) -> &[u64] {
        &self.packer_sorted_granularities
    }

    pub fn min_granularity(&self) -> Result<u64> {
        match self.packer_sorted_granularities.first() {
            Some(min_granularity) => Ok(*min_granularity),
            None => Err(Error::msg("No granularities specified")),
        }
    }

    pub fn start_block(&self) -> u64 {
        self.contracts()
            .iter()
            .map(|c| c.start_block())
            .min()
            .unwrap_or(0)
    }

    pub fn pool_contracts(&self) -> Vec<&PoolContractConfig> {
        self.pool_contract_configs
            .iter()
            .map(|c| c.as_ref())
            .collect()
    }

    pub fn deposit_contracts(&self) -> Vec<&DepositContractConfig> {
        self.deposit_contract_configs
            .iter()
            .map(|c| c.as_ref())
            .collect()
    }

    pub fn deposit_contracts_without_disabled(&self) -> Vec<&DepositContractConfig> {
        self.deposit_contracts()
            .into_iter()
            .filter(|c| !c.disabled())
            .collect()
    }

    pub fn contracts(&self) -> Vec<ContractConfig> {
        let mut contracts: Vec<ContractConfig> = Vec::new();
        for deposit_contract_config in self.deposit_contract_configs.iter() {
            contracts.push(ContractConfig::Deposit(deposit_contract_config.clone()));
        }
        for pool_contract_config in self.pool_contract_configs.iter() {
            contracts.push(ContractConfig::Pool(pool_contract_config.clone()));
        }
        contracts
    }

    pub fn assets(&self) -> Vec<&AssetConfig> {
        self.asset_configs.iter().map(|c| c.as_ref()).collect()
    }

    pub fn supported_asset_symbols(&self) -> Vec<String> {
        let symbols = self
            .pool_contract_configs
            .iter()
            .map(|contract_config| contract_config.asset_symbol().to_string())
            .collect::<HashSet<_>>();
        symbols.into_iter().collect()
    }

    pub fn find_peer_chain_ids(&self) -> Vec<u64> {
        let mut ids: HashSet<u64> = HashSet::new();
        for deposit_contract_config in self.deposit_contracts_without_disabled() {
            if deposit_contract_config.bridge_type() == &BridgeType::Loop {
                ids.insert(self.chain_id());
            } else if let Some(peer_chain_id) = deposit_contract_config.peer_chain_id() {
                ids.insert(*peer_chain_id);
            }
        }
        ids.into_iter().collect()
    }

    pub fn find_asset_symbols(&self, peer_chain_id: u64) -> Vec<&str> {
        let mut asset_symbols: HashSet<&str> = HashSet::new();
        for deposit_contract_config in self.deposit_contracts_without_disabled() {
            if peer_chain_id == self.chain_id() {
                if deposit_contract_config.bridge_type() == &BridgeType::Loop {
                    asset_symbols.insert(deposit_contract_config.asset_symbol());
                }
            } else if deposit_contract_config.bridge_type() != &BridgeType::Loop {
                if let Some(id) = deposit_contract_config.peer_chain_id() {
                    if peer_chain_id == *id {
                        asset_symbols.insert(deposit_contract_config.asset_symbol());
                    }
                }
            }
        }
        asset_symbols.into_iter().collect()
    }

    pub fn find_bridges(&self, peer_chain_id: u64, asset_symbol: &str) -> Vec<&BridgeType> {
        let mut bridges: HashSet<&BridgeType> = HashSet::new();
        for deposit_contract_config in self.deposit_contracts_without_disabled() {
            if peer_chain_id == self.chain_id() {
                if deposit_contract_config.bridge_type() == &BridgeType::Loop
                    && deposit_contract_config.asset_symbol() == asset_symbol
                {
                    return vec![&BridgeType::Loop];
                }
            } else if let Some(id) = deposit_contract_config.peer_chain_id() {
                if *id == peer_chain_id && asset_symbol == deposit_contract_config.asset_symbol() {
                    bridges.insert(deposit_contract_config.bridge_type());
                }
            }
        }
        bridges.into_iter().collect()
    }

    pub fn find_deposit_contract(
        &self,
        peer_chain_id: u64,
        asset_symbol: &str,
        bridge_type: &BridgeType,
    ) -> Option<&DepositContractConfig> {
        for deposit_contract_config in self.deposit_contracts_without_disabled() {
            if deposit_contract_config.asset_symbol() == asset_symbol
                && deposit_contract_config.bridge_type() == bridge_type
            {
                if let Some(id) = deposit_contract_config.peer_chain_id() {
                    if peer_chain_id == *id && bridge_type != &BridgeType::Loop {
                        return Some(deposit_contract_config);
                    }
                }
                if peer_chain_id == self.chain_id() && bridge_type == &BridgeType::Loop {
                    return Some(deposit_contract_config);
                }
            }
        }
        None
    }

    pub fn find_pool_contract(
        &self,
        asset_symbol: &str,
        bridge_type: &BridgeType,
        version: u32,
    ) -> Option<&PoolContractConfig> {
        self.pool_contracts().into_iter().find(|r| {
            r.is_pool_asset(asset_symbol)
                && r.bridge_type() == bridge_type
                && r.version() == version
        })
    }

    pub fn find_pool_contracts(
        &self,
        asset_symbol: &str,
        bridge_type: &BridgeType,
    ) -> Vec<&PoolContractConfig> {
        self.pool_contracts()
            .into_iter()
            .filter(|r| r.is_pool_asset(asset_symbol) && r.bridge_type() == bridge_type)
            .collect()
    }

    pub fn find_pool_contract_by_address(&self, address: &str) -> Option<&PoolContractConfig> {
        self.pool_contracts()
            .into_iter()
            .find(|c| c.address() == address)
    }

    pub fn find_deposit_contract_by_address(
        &self,
        address: &str,
    ) -> Option<&DepositContractConfig> {
        self.deposit_contracts()
            .into_iter()
            .find(|c| c.address() == address)
    }

    pub fn find_contract_by_address(&self, address: &str) -> Option<ContractConfig> {
        if let Some(deposit_contract) = self
            .deposit_contract_configs
            .iter()
            .find(|c| c.address() == address)
        {
            Some(ContractConfig::Deposit(deposit_contract.clone()))
        } else {
            self.pool_contract_configs
                .iter()
                .find(|c| c.address() == address)
                .map(|pool_contract| ContractConfig::Pool(pool_contract.clone()))
        }
    }

    pub fn find_asset(&self, address: &str) -> Option<&AssetConfig> {
        self.assets()
            .into_iter()
            .find(|c| c.asset_address() == address)
    }

    pub fn contract_event_filter_size(&self, address: &str) -> u64 {
        if let Some(deposit_contract) = self.find_deposit_contract_by_address(address) {
            deposit_contract
                .event_filter_size()
                .unwrap_or(self.event_filter_size())
        } else if let Some(pool_contract) = self.find_pool_contract_by_address(address) {
            pool_contract
                .event_filter_size()
                .unwrap_or(self.event_filter_size())
        } else {
            self.event_filter_size()
        }
    }

    pub fn transaction_url(&self, tx_hash: &str) -> String {
        format!("{}{}", self.explorer_url(), self.explorer_prefix())
            .replace(EXPLORER_TX_PLACEHOLDER, tx_hash)
    }

    pub fn validate(&self) -> Result<()> {
        self.raw.validate()?;
        self.main_asset_config.validate()?;
        let mut pool_contracts_versions =
            HashMap::<&str, HashMap<&BridgeType, HashSet<u32>>>::new();
        for pool_contract in self.pool_contracts() {
            pool_contract.validate()?;
            if let Some(pool_contracts_bridges) =
                pool_contracts_versions.get_mut(pool_contract.asset_symbol())
            {
                if let Some(all_versions) =
                    pool_contracts_bridges.get_mut(pool_contract.bridge_type())
                {
                    if all_versions.contains(&pool_contract.version()) {
                        return Err(Error::msg(format!(
                            "only one pool contract is allowed for \
                            chain_id {}, asset_symbol {}, bridge_type {:?} and version {}",
                            self.chain_id(),
                            pool_contract.asset_symbol(),
                            pool_contract.bridge_type(),
                            pool_contract.version(),
                        )));
                    }
                    all_versions.insert(pool_contract.version());
                } else {
                    let mut all_versions: HashSet<u32> = HashSet::new();
                    all_versions.insert(pool_contract.version());
                    pool_contracts_bridges.insert(pool_contract.bridge_type(), all_versions);
                }
            } else {
                let mut all_versions: HashSet<u32> = HashSet::new();
                all_versions.insert(pool_contract.version());
                let mut pool_contract_bridges = HashMap::<&BridgeType, HashSet<u32>>::new();
                pool_contract_bridges.insert(pool_contract.bridge_type(), all_versions);
                pool_contracts_versions.insert(pool_contract.asset_symbol(), pool_contract_bridges);
            }
        }
        for deposit_contract in self.deposit_contracts() {
            deposit_contract.validate()?;
            if let Some(peer_chain_id) = deposit_contract.peer_chain_id() {
                if self.chain_id() == *peer_chain_id {
                    return Err(Error::msg(format!(
                        "deposit contract config at {} chain_id {} \
                        cannot be same as peer_chain_id",
                        deposit_contract.address(),
                        self.chain_id()
                    )));
                }
            }
        }
        for asset_config in self.assets() {
            asset_config.validate()?;
        }
        for provider_config in self.providers() {
            provider_config.validate()?;
        }
        let mut last_granularity: Option<u64> = None;
        for granularity in self.packer_sorted_granularities.iter() {
            if let Some(last_granularity) = last_granularity {
                if granularity % last_granularity != 0 {
                    return Err(Error::msg(format!(
                        "Granularities must be multiples of each other: {granularity} is not a multiple of {last_granularity}"
                    )));
                }
            }
            last_granularity = Some(*granularity);
        }
        Ok(())
    }
}

fn initialize_pool_contracts(
    raw_pool_contracts: &[Arc<RawPoolContractConfig>],
    main_asset_config: &Arc<AssetConfig>,
    asset_configs: &[Arc<AssetConfig>],
    default_circuit_configs: &HashMap<CircuitType, Arc<CircuitConfig>>,
    circuit_configs_by_name: &HashMap<String, Arc<CircuitConfig>>,
) -> Result<Vec<Arc<PoolContractConfig>>> {
    let mut pool_contracts: Vec<Arc<PoolContractConfig>> = Vec::new();
    for raw_pool_contract in raw_pool_contracts {
        let asset_config = if let Some(asset_address) = &raw_pool_contract.asset_address {
            asset_config_by_address(asset_configs, asset_address)?
        } else {
            main_asset_config.clone()
        };
        let mut circuit_configs = HashMap::<CircuitType, Arc<CircuitConfig>>::new();
        for circuit_type in &CircuitType::all() {
            if let Some(circuit_config) = default_circuit_configs.get(circuit_type) {
                circuit_configs.insert(*circuit_type, circuit_config.clone());
            }
        }
        for name in &raw_pool_contract.circuits {
            if let Some(circuit_config) = circuit_configs_by_name.get(name) {
                circuit_configs.insert(*circuit_config.circuit_type(), circuit_config.clone());
            }
        }
        pool_contracts.push(Arc::new(PoolContractConfig::new(
            raw_pool_contract.clone(),
            main_asset_config.clone(),
            asset_config,
            circuit_configs.into_values().collect(),
        )));
    }
    Ok(pool_contracts)
}

fn initialize_deposit_contracts(
    raw_deposit_contracts: &[Arc<RawDepositContractConfig>],
    pool_contracts: &[Arc<PoolContractConfig>],
    main_asset_config: &Arc<AssetConfig>,
    asset_configs: &[Arc<AssetConfig>],
) -> Result<Vec<Arc<DepositContractConfig>>> {
    let mut deposit_contracts: Vec<Arc<DepositContractConfig>> = Vec::new();
    for raw_deposit_contract in raw_deposit_contracts {
        let pool_contract =
            pool_contract_by_address(pool_contracts, &raw_deposit_contract.pool_address)?;
        let bridge_fee_asset = if let Some(bridge_fee_asset_address) =
            &raw_deposit_contract.bridge_fee_asset_address
        {
            asset_config_by_address(asset_configs, bridge_fee_asset_address)?
        } else {
            main_asset_config.clone()
        };
        let executor_fee_asset = if let Some(executor_fee_asset_address) =
            &raw_deposit_contract.executor_fee_asset_address
        {
            asset_config_by_address(asset_configs, executor_fee_asset_address)?
        } else if let Some(asset_address) = pool_contract.asset_address() {
            asset_config_by_address(asset_configs, asset_address)?
        } else {
            main_asset_config.clone()
        };
        deposit_contracts.push(Arc::new(DepositContractConfig::new(
            raw_deposit_contract.clone(),
            bridge_fee_asset,
            executor_fee_asset,
            pool_contract,
        )));
    }
    Ok(deposit_contracts)
}

fn asset_config_by_address(
    asset_configs: &[Arc<AssetConfig>],
    address: &str,
) -> Result<Arc<AssetConfig>> {
    if let Some(asset_config) = asset_configs.iter().find(|c| c.asset_address() == address) {
        Ok(asset_config.clone())
    } else {
        Err(Error::msg(format!("failed to find asset config {address}")))
    }
}

fn pool_contract_by_address(
    pool_contracts: &[Arc<PoolContractConfig>],
    address: &str,
) -> Result<Arc<PoolContractConfig>> {
    if let Some(pool_contract_config) = pool_contracts.iter().find(|c| c.address() == address) {
        Ok(pool_contract_config.clone())
    } else {
        Err(Error::msg(format!(
            "failed to find pool contract {address}"
        )))
    }
}
