use crate::{
    create_raw_from_json, BridgeConfig, ChainConfig, CircuitConfig, ContractConfig,
    DepositContractConfig, PackerConfig, PoolContractConfig, RawChainConfig, RawMystikoConfig,
    SequencerConfig,
};
use anyhow::{Error, Result};
use mystiko_types::{BridgeType, CircuitType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use typed_builder::TypedBuilder;
use validator::Validate;

static DEFAULT_MAINNET_CONFIG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/config/mainnet/config.json"
));
static DEFAULT_TESTNET_CONFIG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/config/testnet/config.json"
));

static DEFAULT_REMOTE_TIMEOUT_MS: u64 = 10000;

#[derive(Clone, Debug)]
pub struct MystikoConfig {
    raw: RawMystikoConfig,
    chain_configs: Vec<Arc<ChainConfig>>,
    circuit_configs: Vec<Arc<CircuitConfig>>,
    bridge_configs: Vec<Arc<BridgeConfig>>,
    sequencer_config: Option<SequencerConfig>,
    packer_config: Option<PackerConfig>,
    default_circuit_configs: HashMap<CircuitType, Arc<CircuitConfig>>,
    circuit_configs_by_name: HashMap<String, Arc<CircuitConfig>>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct MystikoConfigOptions {
    pub file_path: Option<String>,
    pub is_testnet: Option<bool>,
    pub is_staging: Option<bool>,
    pub remote_base_url: Option<String>,
    pub git_revision: Option<String>,
    pub timeout_ms: Option<u64>,
}

const DEFAULT_REMOTE_BASE_URL: &str = "https://static.mystiko.network/config";

impl MystikoConfig {
    fn new(raw: RawMystikoConfig) -> Result<Self> {
        let circuit_configs: Vec<Arc<CircuitConfig>> = raw
            .circuits
            .iter()
            .map(|r| Arc::new(CircuitConfig::new(r.clone())))
            .collect();
        let bridge_configs: Vec<Arc<BridgeConfig>> = raw
            .bridges
            .iter()
            .map(|r| Arc::new(BridgeConfig::new(r.clone())))
            .collect();
        let packer_config = raw.packer.as_ref().map(|r| PackerConfig::new(r.clone()));
        let sequencer_config = raw
            .sequencer
            .as_ref()
            .map(|r| SequencerConfig::new(r.clone()));
        let default_circuit_configs = initialize_default_circuit_configs(&circuit_configs)?;
        let circuit_configs_by_name = initialize_circuit_configs_by_name(&circuit_configs)?;
        let chain_configs = initialize_chain_configs(
            &raw.chains,
            &default_circuit_configs,
            &circuit_configs_by_name,
        )?;
        Ok(MystikoConfig {
            raw,
            chain_configs,
            bridge_configs,
            circuit_configs,
            packer_config,
            sequencer_config,
            default_circuit_configs,
            circuit_configs_by_name,
        })
    }

    pub fn from_raw(raw: RawMystikoConfig) -> Result<Self> {
        let config = MystikoConfig::new(raw)?;
        config.validate()?;
        Ok(config)
    }

    pub fn from_json_str(json: &str) -> Result<Self> {
        MystikoConfig::from_raw(create_raw_from_json::<RawMystikoConfig>(json)?)
    }

    #[cfg(feature = "fs")]
    pub async fn from_json_file(json_file: &str) -> Result<Self> {
        MystikoConfig::from_raw(crate::create_raw_from_file::<RawMystikoConfig>(json_file).await?)
    }

    pub async fn from_remote(options: &MystikoConfigOptions) -> Result<Self> {
        let url = Self::remote_url(options);
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(
                options.timeout_ms.unwrap_or(DEFAULT_REMOTE_TIMEOUT_MS),
            ))
            .build()?;
        let response = client.get(&url).send().await?;
        if response.status().is_success() {
            let content = response.text().await?;
            MystikoConfig::from_json_str(&content)
        } else {
            Err(Error::msg(format!(
                "Failed to fetch config from {}: status code {}",
                &url,
                response.status()
            )))
        }
    }

    pub async fn from_remote_default_mainnet() -> Result<Self> {
        MystikoConfig::from_remote(&MystikoConfigOptions::default()).await
    }

    pub async fn from_remote_default_testnet() -> Result<Self> {
        MystikoConfig::from_remote(&MystikoConfigOptions::builder().is_testnet(true).build()).await
    }

    pub fn from_local_default_mainnet() -> Result<Self> {
        MystikoConfig::from_json_str(DEFAULT_MAINNET_CONFIG)
    }

    pub fn from_local_default_testnet() -> Result<Self> {
        MystikoConfig::from_json_str(DEFAULT_TESTNET_CONFIG)
    }

    pub async fn from_default_mainnet() -> Result<Self> {
        match MystikoConfig::from_remote_default_mainnet().await {
            Ok(config) => Ok(config),
            Err(err) => {
                log::warn!(
                    "Failed to build default mainnet config from remote: {}",
                    err
                );
                MystikoConfig::from_local_default_mainnet()
            }
        }
    }

    pub async fn from_default_testnet() -> Result<Self> {
        match MystikoConfig::from_remote_default_testnet().await {
            Ok(config) => Ok(config),
            Err(err) => {
                log::warn!(
                    "Failed to build default testnet config from remote: {}",
                    err
                );
                MystikoConfig::from_local_default_testnet()
            }
        }
    }

    pub async fn from_options<O>(options: O) -> Result<Self>
    where
        O: Into<MystikoConfigOptions>,
    {
        let options: MystikoConfigOptions = options.into();
        #[cfg(feature = "fs")]
        match &options.file_path {
            Some(path) => MystikoConfig::from_json_file(path).await,
            None => MystikoConfig::from_remote(&options).await,
        }
        #[cfg(not(feature = "fs"))]
        MystikoConfig::from_remote(&options).await
    }

    pub fn remote_url(options: &MystikoConfigOptions) -> String {
        let base_url = options
            .remote_base_url
            .clone()
            .unwrap_or_else(|| DEFAULT_REMOTE_BASE_URL.to_string());
        let environment =
            options
                .is_staging
                .map_or("production", |s| if s { "staging" } else { "production" });
        let network = options
            .is_testnet
            .map_or("mainnet", |s| if s { "testnet" } else { "mainnet" });

        if let Some(git_revision) = &options.git_revision {
            format!(
                "{}/{}/{}/{}/config.json",
                base_url, environment, network, git_revision
            )
        } else {
            format!("{}/{}/{}/latest.json", base_url, environment, network)
        }
    }

    pub fn version(&self) -> &str {
        &self.raw.version
    }

    pub fn git_revision(&self) -> Option<&str> {
        self.raw.git_revision.as_deref()
    }

    pub fn circuits(&self) -> Vec<&CircuitConfig> {
        self.circuit_configs.iter().map(|c| c.as_ref()).collect()
    }

    pub fn bridges(&self) -> Vec<&BridgeConfig> {
        self.bridge_configs.iter().map(|c| c.as_ref()).collect()
    }

    pub fn chains(&self) -> Vec<&ChainConfig> {
        self.chain_configs.iter().map(|c| c.as_ref()).collect()
    }

    pub fn sequencer(&self) -> Option<&SequencerConfig> {
        self.sequencer_config.as_ref()
    }

    pub fn packer(&self) -> Option<&PackerConfig> {
        self.packer_config.as_ref()
    }

    pub fn country_blacklist(&self) -> Vec<&str> {
        self.raw
            .country_blacklist
            .iter()
            .map(|c| c.as_str())
            .collect()
    }

    pub fn find_default_circuit(&self, circuit_type: &CircuitType) -> Option<&CircuitConfig> {
        self.default_circuit_configs
            .get(circuit_type)
            .map(|c| c.as_ref())
    }

    pub fn find_circuit(&self, circuit_name: &str) -> Option<&CircuitConfig> {
        self.circuit_configs_by_name
            .get(circuit_name)
            .map(|c| c.as_ref())
    }

    pub fn find_chain(&self, chain_id: u64) -> Option<&ChainConfig> {
        self.chains().into_iter().find(|c| c.chain_id() == chain_id)
    }

    pub fn find_peer_chains(&self, chain_id: u64) -> Vec<&ChainConfig> {
        let mut peer_chains: Vec<&ChainConfig> = Vec::new();
        if let Some(peer_chain_ids) = self.find_chain(chain_id).map(|c| c.find_peer_chain_ids()) {
            for peer_chain_id in peer_chain_ids {
                if let Some(peer_chain) = self.find_chain(peer_chain_id) {
                    peer_chains.push(peer_chain);
                }
            }
        }
        peer_chains
    }

    pub fn find_asset_symbols(&self, chain_id: u64, peer_chain_id: u64) -> Vec<&str> {
        self.find_chain(chain_id)
            .map(|c| c.find_asset_symbols(peer_chain_id))
            .unwrap_or_default()
    }

    pub fn find_bridges(
        &self,
        chain_id: u64,
        peer_chain_id: u64,
        asset_symbol: &str,
    ) -> Vec<&BridgeType> {
        self.find_chain(chain_id)
            .map(|c| c.find_bridges(peer_chain_id, asset_symbol))
            .unwrap_or_default()
    }

    pub fn find_bridge(&self, bridge_type: &BridgeType) -> Option<&BridgeConfig> {
        self.bridges()
            .into_iter()
            .find(|c| c.bridge_type() == bridge_type)
    }

    pub fn find_deposit_contract(
        &self,
        chain_id: u64,
        peer_chain_id: u64,
        asset_symbol: &str,
        bridge_type: &BridgeType,
    ) -> Option<&DepositContractConfig> {
        if let Some(chain_config) = self.find_chain(chain_id) {
            chain_config.find_deposit_contract(peer_chain_id, asset_symbol, bridge_type)
        } else {
            None
        }
    }

    pub fn find_deposit_contract_by_address(
        &self,
        chain_id: u64,
        address: &str,
    ) -> Option<&DepositContractConfig> {
        if let Some(chain_config) = self.find_chain(chain_id) {
            chain_config.find_deposit_contract_by_address(address)
        } else {
            None
        }
    }

    pub fn find_pool_contracts(
        &self,
        chain_id: u64,
        asset_symbol: &str,
        bridge_type: &BridgeType,
    ) -> Vec<&PoolContractConfig> {
        self.find_chain(chain_id)
            .map(|c| c.find_pool_contracts(asset_symbol, bridge_type))
            .unwrap_or_default()
    }

    pub fn find_pool_contract(
        &self,
        chain_id: u64,
        asset_symbol: &str,
        bridge_type: &BridgeType,
        version: u32,
    ) -> Option<&PoolContractConfig> {
        if let Some(chain_config) = self.find_chain(chain_id) {
            chain_config.find_pool_contract(asset_symbol, bridge_type, version)
        } else {
            None
        }
    }

    pub fn find_pool_contract_by_address(
        &self,
        chain_id: u64,
        address: &str,
    ) -> Option<&PoolContractConfig> {
        if let Some(chain_config) = self.find_chain(chain_id) {
            chain_config.find_pool_contract_by_address(address)
        } else {
            None
        }
    }

    pub fn find_contract_by_address(&self, chain_id: u64, address: &str) -> Option<ContractConfig> {
        if let Some(chain_config) = self.find_chain(chain_id) {
            chain_config.find_contract_by_address(address)
        } else {
            None
        }
    }

    pub fn transaction_url(&self, chain_id: u64, tx_hash: &str) -> Option<String> {
        self.find_chain(chain_id)
            .map(|c| c.transaction_url(tx_hash))
    }

    pub fn validate(&self) -> Result<()> {
        self.raw.validate()?;
        for chain_config in self.chains() {
            chain_config.validate()?;
        }
        for chain_config in self.chains() {
            for deposit_contract_config in chain_config.deposit_contracts() {
                if deposit_contract_config.bridge_type() != &BridgeType::Loop
                    && self
                        .find_bridge(deposit_contract_config.bridge_type())
                        .is_none()
                {
                    return Err(Error::msg(format!(
                        "no bridge config for bridge_type {:?}",
                        deposit_contract_config.bridge_type()
                    )));
                }
                if let (Some(peer_chain_id), Some(peer_chain_address)) = (
                    deposit_contract_config.peer_chain_id(),
                    deposit_contract_config.peer_contract_address(),
                ) {
                    if self.find_chain(*peer_chain_id).is_some() {
                        if let Some(peer_contract) = self
                            .find_deposit_contract_by_address(*peer_chain_id, peer_chain_address)
                        {
                            if peer_contract.bridge_type() != deposit_contract_config.bridge_type()
                            {
                                return Err(Error::msg(format!(
                                    "mismatched bridge_types {:?} vs {:?} \
                                    for peer deposit contract config of \
                                    chain_id {} at {} \
                                    for deposit contract config of chain_id {} at {}",
                                    peer_contract.bridge_type(),
                                    deposit_contract_config.bridge_type(),
                                    peer_chain_id,
                                    peer_chain_address,
                                    chain_config.chain_id(),
                                    deposit_contract_config.address()
                                )));
                            }
                            if peer_contract.peer_chain_id() != &Some(chain_config.chain_id()) {
                                return Err(Error::msg(format!(
                                    "peer_chain_id for peer deposit contract config of \
                                    chain_id {} at {} \
                                    should be {}",
                                    peer_chain_id,
                                    peer_chain_address,
                                    chain_config.chain_id(),
                                )));
                            }
                            if peer_contract.peer_contract_address()
                                != Some(deposit_contract_config.address())
                            {
                                return Err(Error::msg(format!(
                                    "peer_contract_address for peer deposit contract config of \
                                    chain_id {} at {} \
                                    should be {}",
                                    peer_chain_id,
                                    peer_chain_address,
                                    deposit_contract_config.address(),
                                )));
                            }
                        } else {
                            return Err(Error::msg(format!(
                                "cannot find peer deposit contract config of \
                                chain_id {} at {} \
                                for deposit contract config of chain_id {} at {}",
                                peer_chain_id,
                                peer_chain_address,
                                chain_config.chain_id(),
                                deposit_contract_config.address(),
                            )));
                        }
                    } else {
                        return Err(Error::msg(format!(
                            "cannot find chain config of peer_chain_id {} \
                            for deposit contract config at {} chain_id {}",
                            peer_chain_id,
                            deposit_contract_config.address(),
                            chain_config.chain_id()
                        )));
                    }
                }
            }
        }
        for circuit_config in self.circuits() {
            circuit_config.validate()?;
        }
        Ok(())
    }
}

fn initialize_chain_configs(
    raw_chain_configs: &[Arc<RawChainConfig>],
    default_circuit_configs: &HashMap<CircuitType, Arc<CircuitConfig>>,
    circuit_configs_by_name: &HashMap<String, Arc<CircuitConfig>>,
) -> Result<Vec<Arc<ChainConfig>>> {
    let mut chain_configs: Vec<Arc<ChainConfig>> = Vec::new();
    for raw_chain_config in raw_chain_configs {
        chain_configs.push(Arc::new(ChainConfig::new(
            raw_chain_config.clone(),
            default_circuit_configs,
            circuit_configs_by_name,
        )?));
    }
    Ok(chain_configs)
}

fn initialize_default_circuit_configs(
    circuit_configs: &[Arc<CircuitConfig>],
) -> Result<HashMap<CircuitType, Arc<CircuitConfig>>> {
    let mut configs: HashMap<CircuitType, Arc<CircuitConfig>> = HashMap::new();
    for circuit_config in circuit_configs.iter() {
        if circuit_config.is_default() {
            if configs.contains_key(circuit_config.circuit_type()) {
                return Err(Error::msg(format!(
                    "duplicate default circuit config for circuit_type {:?}",
                    circuit_config.circuit_type()
                )));
            }
            configs.insert(*circuit_config.circuit_type(), circuit_config.clone());
        }
    }
    Ok(configs)
}

fn initialize_circuit_configs_by_name(
    circuit_configs: &[Arc<CircuitConfig>],
) -> Result<HashMap<String, Arc<CircuitConfig>>> {
    let mut configs: HashMap<String, Arc<CircuitConfig>> = HashMap::new();
    for circuit_config in circuit_configs.iter() {
        if configs.contains_key(circuit_config.name()) {
            return Err(Error::msg(format!(
                "duplicate circuit config for name {}",
                circuit_config.name()
            )));
        }
        configs.insert(circuit_config.name().to_string(), circuit_config.clone());
    }
    Ok(configs)
}
