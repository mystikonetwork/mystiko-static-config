use mystiko_config::{
    create_raw_from_file, ChainConfig, CircuitConfig, RawChainConfig, RawCircuitConfig,
    MAIN_ASSET_ADDRESS,
};
use mystiko_types::{AssetType, BridgeType, CircuitType, ProviderType, TransactionType};
use num_bigint::BigUint;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tokio::fs::read_to_string;

const VALID_CONFIG_FILE: &str = "tests/files/chain/valid.json";
const FULL_CONFIG_FILE: &str = "tests/files/chain/full.json";

#[tokio::test]
async fn test_create() {
    let (_, _, _, config) = setup(SetupOptions::default()).await;
    config.validate().unwrap();
    assert_eq!(config.chain_id(), 5);
    assert_eq!(config.name(), "Ethereum Goerli");
    assert_eq!(config.main_asset().asset_address(), MAIN_ASSET_ADDRESS);
    assert_eq!(config.main_asset().asset_type(), &AssetType::Main);
    assert_eq!(config.main_asset().asset_symbol(), "ETH");
    assert_eq!(config.main_asset().asset_decimals(), 18);
    assert_eq!(
        config.main_asset().recommended_amounts().unwrap(),
        vec![
            BigUint::from_str("1000000000000000000").unwrap(),
            BigUint::from_str("10000000000000000000").unwrap()
        ]
    );
    assert_eq!(
        config
            .main_asset()
            .recommended_amounts_number::<u32>()
            .unwrap(),
        vec![1, 10]
    );
    assert_eq!(config.asset_symbol(), config.main_asset().asset_symbol());
    assert_eq!(
        config.asset_decimals(),
        config.main_asset().asset_decimals()
    );
    assert_eq!(
        config.recommended_amounts().unwrap(),
        config.main_asset().recommended_amounts().unwrap()
    );
    assert_eq!(
        config.recommended_amounts_number::<u32>().unwrap(),
        config
            .main_asset()
            .recommended_amounts_number::<u32>()
            .unwrap()
    );
    assert_eq!(config.explorer_url(), "https://goerli.etherscan.io");
    assert_eq!(config.explorer_api_url(), "https://api-goerli.etherscan.io");
    assert_eq!(config.explorer_prefix(), "/tx/%tx%");
    assert_eq!(config.event_delay_blocks(), 200);
    assert_eq!(config.event_filter_size(), 1000);
    assert_eq!(config.sequencer_fetch_size(), 20000);
    assert_eq!(
        config.signer_endpoint(),
        "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161"
    );
    assert_eq!(
        &config.transaction_url(
            "0xc478553f5920534885934acd4c248904\
            70f22469f79b0c34abc8a258af4dcd77"
        ),
        "https://goerli.etherscan.io/tx/\
        0xc478553f5920534885934acd4c24890470f22469f79b0c34abc8a258af4dcd77"
    );
    assert_eq!(config.assets().len(), 1);
    assert_eq!(config.supported_asset_symbols(), vec!["MTT"]);
    assert_eq!(config.granularities(), &vec![2000, 4000, 8000, 16000]);
    assert_eq!(config.min_granularity().unwrap(), 2000);
    assert_eq!(config.start_block(), 1000000);
    assert_eq!(config.providers().len(), 1);
    assert_eq!(config.provider_type(), &ProviderType::Quorum);
    assert_eq!(config.provider_quorum_percentage(), 80);
    assert_eq!(config.transaction_type(), &TransactionType::Legacy);
    assert_eq!(config.pool_contracts().len(), 1);
    assert_eq!(config.deposit_contracts_without_disabled().len(), 0);
    assert_eq!(config.deposit_contracts().len(), 1);
    assert_eq!(config.contracts().len(), 2);
    assert_eq!(config.safe_confirmations().unwrap(), 10);
    let asset = *config.assets().first().unwrap();
    let provider = *config.providers().first().unwrap();
    let pool_contract = *config.pool_contracts().first().unwrap();
    let deposit_contract = *config.deposit_contracts().first().unwrap();
    assert_eq!(
        asset.asset_address(),
        "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a"
    );
    assert_eq!(
        provider.url(),
        "wss://goerli.infura.io/ws/v3/9aa3d95b3bc440fa88ea12eaa4456161"
    );
    assert_eq!(
        pool_contract.address(),
        "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"
    );
    assert_eq!(
        deposit_contract.address(),
        "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
    );
}

#[tokio::test]
async fn test_selectors() {
    let (_, _, _, config) = setup(SetupOptions {
        full_config: true,
        ..SetupOptions::default()
    })
    .await;
    config.validate().unwrap();
    assert_eq!(config.contracts().len(), 12);
    let mut peer_chain_ids = config.find_peer_chain_ids();
    peer_chain_ids.sort();
    assert_eq!(peer_chain_ids, vec![5, 97]);
    assert_eq!(config.find_asset_symbols(5), vec!["ETH"]);
    assert_eq!(config.find_asset_symbols(97), vec!["MTT"]);
    assert!(config.find_asset_symbols(1).is_empty());
    assert_eq!(config.find_bridges(5, "ETH"), vec![&BridgeType::Loop]);
    assert!(config.find_bridges(5, "MTT").is_empty());
    let mut bridges = config.find_bridges(97, "MTT");
    bridges.sort_by_key(|b| format!("{:?}", b));
    assert_eq!(
        bridges,
        vec![
            &BridgeType::Axelar,
            &BridgeType::Celer,
            &BridgeType::LayerZero,
            &BridgeType::Tbridge
        ]
    );
    let deposit_contract1 = config
        .find_deposit_contract(5, "ETH", &BridgeType::Loop)
        .unwrap();
    assert_eq!(
        deposit_contract1.address(),
        "0x390d485f4d43212d4ae8cdd967a711514ed5a54f"
    );
    let deposit_contract2 = config.find_deposit_contract(5, "MTT", &BridgeType::Loop);
    assert!(deposit_contract2.is_none());
    let deposit_contract3 = config
        .find_deposit_contract(97, "MTT", &BridgeType::Axelar)
        .unwrap();
    assert_eq!(
        deposit_contract3.address(),
        "0xF0850b2a2EC6e9E214D077658A95c4745a8B8aAf"
    );
    assert!(config
        .find_deposit_contract_by_address("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")
        .is_none());
    assert!(config
        .find_deposit_contract_by_address("0xF0850b2a2EC6e9E214D077658A95c4745a8B8aAf")
        .is_some());
    assert!(config
        .find_contract_by_address("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")
        .is_none());
    assert!(config
        .find_contract_by_address("0xF0850b2a2EC6e9E214D077658A95c4745a8B8aAf")
        .is_some());
    let mut pool_contracts = config.find_pool_contracts("MTT", &BridgeType::Tbridge);
    pool_contracts.sort_by_key(|c| c.version());
    assert_eq!(pool_contracts.len(), 2);
    let pool_contract1 = *pool_contracts.first().unwrap();
    let pool_contract2 = *pool_contracts.get(1).unwrap();
    assert_eq!(
        pool_contract1.address(),
        "0x9b42ec45f6fb6c7d252c66741e960585888de7b6"
    );
    assert_eq!(
        pool_contract2.address(),
        "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"
    );
    assert_eq!(
        config
            .find_pool_contract("MTT", &BridgeType::Tbridge, 1)
            .unwrap()
            .address(),
        "0x9b42ec45f6fb6c7d252c66741e960585888de7b6"
    );
    assert!(config
        .find_pool_contract("MTT", &BridgeType::Tbridge, 3)
        .is_none());
    assert!(config
        .find_pool_contract_by_address("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")
        .is_none());
    assert!(config
        .find_pool_contract_by_address("0x9b42ec45f6fb6c7d252c66741e960585888de7b6")
        .is_some());
    assert!(config
        .find_contract_by_address("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")
        .is_none());
    assert!(config
        .find_contract_by_address("0x9b42ec45f6fb6c7d252c66741e960585888de7b6")
        .is_some());
    assert!(config
        .find_asset("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")
        .is_some());
    assert!(config
        .find_asset("0x9b42ec45f6fb6c7d252c66741e960585888de7b6")
        .is_none());
}

#[tokio::test]
async fn test_event_filter_size() {
    let (_, _, _, config) = setup(SetupOptions {
        full_config: true,
        ..SetupOptions::default()
    })
    .await;
    assert_eq!(
        config.contract_event_filter_size("0x7b5753f81f73d160583083c33b0f863837197fb3"),
        1000
    );
    assert_eq!(
        config.contract_event_filter_size("0xbF5605f5Ed6d18ed957cBA80dbA8838dFcb9A69f"),
        1000
    );
    assert_eq!(
        config.contract_event_filter_size("0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"),
        2000
    );
    assert_eq!(
        config.contract_event_filter_size("0x9b42ec45f6fb6c7d252c66741e960585888de7b6"),
        3000
    );
}

#[tokio::test]
async fn test_validate_duplicate_pool_contract_version() {
    let (_, _, _, config) = setup(SetupOptions {
        duplicate_pool_contract_version: true,
        ..SetupOptions::default()
    })
    .await;
    assert_eq!(
        config.validate().err().unwrap().to_string(),
        "only one pool contract is allowed for \
        chain_id 5, asset_symbol MTT, bridge_type Tbridge and version 2"
    );
}

#[tokio::test]
async fn test_validate_invalid_peer_chain_id() {
    let (_, _, _, config) = setup(SetupOptions {
        invalid_peer_chain_id: true,
        ..SetupOptions::default()
    })
    .await;
    assert_eq!(
        config.validate().err().unwrap().to_string(),
        "deposit contract config at 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 \
        chain_id 5 cannot be same as peer_chain_id"
    );
}

#[tokio::test]
async fn test_missing_bridge_asset_config() {
    let (default_circuit_configs, circuit_configs_by_name, raw_config, _) =
        setup(SetupOptions::default()).await;
    let mut new_raw_config = raw_config.as_ref().clone();
    let mut deposit_contract_config = new_raw_config.deposit_contracts.remove(0).as_ref().clone();
    deposit_contract_config.bridge_fee_asset_address =
        Some(String::from("0xe688b84b23f322a994a53dbf8e15fa82cdb71127"));
    new_raw_config
        .deposit_contracts
        .push(Arc::new(deposit_contract_config));
    assert_eq!(
        ChainConfig::new(
            Arc::new(new_raw_config),
            default_circuit_configs.as_ref(),
            circuit_configs_by_name.as_ref()
        )
        .err()
        .unwrap()
        .to_string(),
        "failed to find asset config 0xe688b84b23f322a994a53dbf8e15fa82cdb71127"
    );
}

#[tokio::test]
async fn test_missing_executor_asset_config() {
    let (default_circuit_configs, circuit_configs_by_name, raw_config, _) =
        setup(SetupOptions::default()).await;
    let mut new_raw_config = raw_config.as_ref().clone();
    let mut deposit_contract_config = new_raw_config.deposit_contracts.remove(0).as_ref().clone();
    deposit_contract_config.executor_fee_asset_address =
        Some(String::from("0xe688b84b23f322a994a53dbf8e15fa82cdb71127"));
    new_raw_config
        .deposit_contracts
        .push(Arc::new(deposit_contract_config));
    assert_eq!(
        ChainConfig::new(
            Arc::new(new_raw_config),
            default_circuit_configs.as_ref(),
            circuit_configs_by_name.as_ref()
        )
        .err()
        .unwrap()
        .to_string(),
        "failed to find asset config 0xe688b84b23f322a994a53dbf8e15fa82cdb71127"
    );
}

#[tokio::test]
async fn test_missing_pool_contract_config() {
    let (default_circuit_configs, circuit_configs_by_name, raw_config, _) =
        setup(SetupOptions::default()).await;
    let mut new_raw_config = raw_config.as_ref().clone();
    let mut deposit_contract_config = new_raw_config.deposit_contracts.remove(0).as_ref().clone();
    deposit_contract_config.pool_address =
        String::from("0xe688b84b23f322a994a53dbf8e15fa82cdb71127");
    new_raw_config
        .deposit_contracts
        .push(Arc::new(deposit_contract_config));
    assert_eq!(
        ChainConfig::new(
            Arc::new(new_raw_config),
            default_circuit_configs.as_ref(),
            circuit_configs_by_name.as_ref()
        )
        .err()
        .unwrap()
        .to_string(),
        "failed to find pool contract 0xe688b84b23f322a994a53dbf8e15fa82cdb71127"
    );
}

#[derive(Default)]
struct SetupOptions {
    full_config: bool,
    duplicate_pool_contract_version: bool,
    invalid_peer_chain_id: bool,
}

async fn setup(
    options: SetupOptions,
) -> (
    Arc<HashMap<CircuitType, Arc<CircuitConfig>>>,
    Arc<HashMap<String, Arc<CircuitConfig>>>,
    Arc<RawChainConfig>,
    ChainConfig,
) {
    let mut raw_config = if options.full_config {
        create_raw_from_file::<RawChainConfig>(FULL_CONFIG_FILE)
            .await
            .unwrap()
    } else {
        create_raw_from_file::<RawChainConfig>(VALID_CONFIG_FILE)
            .await
            .unwrap()
    };
    if options.duplicate_pool_contract_version {
        let mut raw_pool_contract = raw_config.pool_contracts.first().unwrap().as_ref().clone();
        raw_pool_contract.address = String::from("0x7b5753f81f73d160583083c33b0f863837197fb3");
        raw_config.pool_contracts.push(Arc::new(raw_pool_contract));
    }
    if options.invalid_peer_chain_id {
        let mut raw_deposit_contract = raw_config.deposit_contracts.remove(0).as_ref().clone();
        raw_deposit_contract.peer_chain_id = Some(5);
        raw_config
            .deposit_contracts
            .push(Arc::new(raw_deposit_contract));
    }
    let raw_config_arc = Arc::new(raw_config);
    let raw_circuit_configs: Vec<Arc<CircuitConfig>> = create_raw_circuit_configs()
        .await
        .into_iter()
        .map(|r| Arc::new(CircuitConfig::new(Arc::new(r))))
        .collect();
    let mut default_circuit_configs: HashMap<CircuitType, Arc<CircuitConfig>> = HashMap::new();
    let mut circuit_configs_by_name: HashMap<String, Arc<CircuitConfig>> = HashMap::new();
    for circuit_config in &raw_circuit_configs {
        if circuit_config.is_default() {
            default_circuit_configs.insert(*circuit_config.circuit_type(), circuit_config.clone());
        }
        circuit_configs_by_name.insert(circuit_config.name().to_string(), circuit_config.clone());
    }
    let default_circuit_configs_arc = Arc::new(default_circuit_configs);
    let circuit_configs_by_name_arc = Arc::new(circuit_configs_by_name);
    let config = ChainConfig::new(
        raw_config_arc.clone(),
        default_circuit_configs_arc.as_ref(),
        circuit_configs_by_name_arc.as_ref(),
    )
    .unwrap();
    (
        default_circuit_configs_arc,
        circuit_configs_by_name_arc,
        raw_config_arc,
        config,
    )
}

async fn create_raw_circuit_configs() -> Vec<RawCircuitConfig> {
    let contents = read_to_string(PathBuf::from("tests/files/chain/circuits.json"))
        .await
        .unwrap();
    serde_json::from_str::<Vec<RawCircuitConfig>>(&contents).unwrap()
}

#[tokio::test]
async fn test_invalid_granularities() {
    let (default_circuit_configs, circuit_configs, raw_config, _) =
        setup(SetupOptions::default()).await;
    let mut raw_config = raw_config.as_ref().clone();
    raw_config.packer_granularities = vec![1000u64, 1010u64];
    let config = ChainConfig::new(
        Arc::new(raw_config),
        default_circuit_configs.as_ref(),
        circuit_configs.as_ref(),
    )
    .unwrap();
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_empty_granularities() {
    let (default_circuit_configs, circuit_configs, raw_config, _) =
        setup(SetupOptions::default()).await;
    let mut raw_config = raw_config.as_ref().clone();
    raw_config.packer_granularities = vec![];
    let config = ChainConfig::new(
        Arc::new(raw_config),
        default_circuit_configs.as_ref(),
        circuit_configs.as_ref(),
    )
    .unwrap();
    assert!(config.min_granularity().is_err());
}
