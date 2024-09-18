use mystiko_config::{
    create_raw_from_file, ChainConfig, CircuitConfig, RawChainConfig, RawCircuitConfig,
};
use mystiko_types::{BridgeType, CircuitType};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::read_to_string;

const VALID_CONFIG_FILE: &str = "tests/files/chain/valid.alias.json";

#[tokio::test]
async fn test_selectors() {
    let (_, _, _, config) = setup().await;
    config.validate().unwrap();
    assert_eq!(config.contracts().len(), 5);
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
    assert_eq!(bridges, vec![&BridgeType::Tbridge]);
    let deposit_contract1 = config
        .find_deposit_contract(97, "MTT", &BridgeType::Tbridge)
        .unwrap();
    assert_eq!(
        deposit_contract1.address(),
        "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
    );
    let deposit_contract2 = config.find_deposit_contract(5, "MTT", &BridgeType::Loop);
    assert!(deposit_contract2.is_none());
    let deposit_contract3 = config
        .find_deposit_contract(5, "ETH", &BridgeType::Loop)
        .unwrap();
    assert_eq!(
        deposit_contract3.address(),
        "0xd791049D0a154bC7860804e1A18ACD148Eb0afD9"
    );
    assert!(config
        .find_deposit_contract_by_address("0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d")
        .is_none());
    assert!(config
        .find_deposit_contract_by_address("0xd791049D0a154bC7860804e1A18ACD148Eb0afD9")
        .is_some());
    assert!(config
        .find_contract_by_address("0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d")
        .is_some());
    assert!(config
        .find_contract_by_address("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")
        .is_none());

    for symbol in ["ETH", "wETH", "stETH"] {
        let mut pool_contracts = config.find_pool_contracts(symbol, &BridgeType::Loop);
        pool_contracts.sort_by_key(|c| c.version());
        assert_eq!(pool_contracts.len(), 2);
        let pool_contract1 = *pool_contracts.first().unwrap();
        let pool_contract2 = *pool_contracts.get(1).unwrap();
        assert_eq!(
            pool_contract1.address(),
            "0xBe2C9c8a00951662dF3a978b25F448968F0595AE"
        );
        assert_eq!(
            pool_contract2.address(),
            "0xCFC94003081ce7EcdBc43f94A443Cf9fad0F8847"
        );
        assert_eq!(
            config
                .find_pool_contract(symbol, &BridgeType::Loop, 2)
                .unwrap()
                .address(),
            "0xBe2C9c8a00951662dF3a978b25F448968F0595AE"
        );
        assert_eq!(
            config
                .find_pool_contract(symbol, &BridgeType::Loop, 3)
                .unwrap()
                .address(),
            "0xCFC94003081ce7EcdBc43f94A443Cf9fad0F8847"
        );
    }

    for symbol in ["MTT", "wMTT", "stMTT"] {
        let mut pool_contracts = config.find_pool_contracts(symbol, &BridgeType::Tbridge);
        pool_contracts.sort_by_key(|c| c.version());
        assert_eq!(pool_contracts.len(), 1);
        let pool_contract1 = *pool_contracts.first().unwrap();
        assert_eq!(
            pool_contract1.address(),
            "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"
        );
        assert_eq!(
            config
                .find_pool_contract(symbol, &BridgeType::Tbridge, 2)
                .unwrap()
                .address(),
            "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"
        );

        assert!(config
            .find_pool_contract(symbol, &BridgeType::Tbridge, 3)
            .is_none());
    }

    assert!(config
        .find_pool_contract_by_address("0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d")
        .is_some());
    assert!(config
        .find_pool_contract_by_address("0xBe2C9c8a00951662dF3a978b25F448968F0595AE")
        .is_some());
    assert!(config
        .find_pool_contract_by_address("0xCFC94003081ce7EcdBc43f94A443Cf9fad0F8847")
        .is_some());
    assert!(config
        .find_contract_by_address("0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d")
        .is_some());
    assert!(config
        .find_contract_by_address("0xBe2C9c8a00951662dF3a978b25F448968F0595AE")
        .is_some());
    assert!(config
        .find_contract_by_address("0xCFC94003081ce7EcdBc43f94A443Cf9fad0F8847")
        .is_some());

    assert!(config
        .find_asset("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")
        .is_some());
    assert!(config
        .find_asset("0xCFC94003081ce7EcdBc43f94A443Cf9fad0F8847")
        .is_none());
}

async fn setup() -> (
    Arc<HashMap<CircuitType, Arc<CircuitConfig>>>,
    Arc<HashMap<String, Arc<CircuitConfig>>>,
    Arc<RawChainConfig>,
    ChainConfig,
) {
    let raw_config = create_raw_from_file::<RawChainConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
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
