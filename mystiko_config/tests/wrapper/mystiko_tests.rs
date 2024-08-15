use std::sync::Arc;

use mockito::Server;

use mystiko_config::{create_raw_from_file, MystikoConfig, MystikoConfigOptions, RawMystikoConfig};
use mystiko_types::{BridgeType, CircuitType};

const VALID_CONFIG_FILE: &str = "tests/files/mystiko/valid.json";
const FULL_CONFIG_FILE: &str = "tests/files/mystiko/full.json";

#[tokio::test]
async fn test_create() {
    let config = MystikoConfig::from_json_file(VALID_CONFIG_FILE)
        .await
        .unwrap();
    config.validate().unwrap();
    assert_eq!(config.version(), "0.1.0");
    assert_eq!(config.git_revision().unwrap(), "b6b5b5b");
    let mut chain_ids: Vec<u64> = config.chains().into_iter().map(|c| c.chain_id()).collect();
    chain_ids.sort();
    assert_eq!(chain_ids, vec![5, 97]);
    let mut bridge_names: Vec<&str> = config.bridges().into_iter().map(|b| b.name()).collect();
    bridge_names.sort();
    assert_eq!(
        bridge_names,
        vec![
            "Axelar Network",
            "Celer Network",
            "LayerZero Bridge",
            "Mystiko Testnet Bridge",
            "Poly Bridge"
        ]
    );
    let mut circuit_names: Vec<&str> = config.circuits().into_iter().map(|c| c.name()).collect();
    circuit_names.sort();
    assert_eq!(
        circuit_names,
        vec![
            "zokrates-1.0-rollup1",
            "zokrates-1.0-rollup1024",
            "zokrates-1.0-rollup128",
            "zokrates-1.0-rollup16",
            "zokrates-1.0-rollup2",
            "zokrates-1.0-rollup256",
            "zokrates-1.0-rollup32",
            "zokrates-1.0-rollup4",
            "zokrates-1.0-rollup512",
            "zokrates-1.0-rollup64",
            "zokrates-1.0-rollup8",
            "zokrates-1.0-transaction1x0",
            "zokrates-1.0-transaction1x1",
            "zokrates-1.0-transaction1x2",
            "zokrates-1.0-transaction2x0",
            "zokrates-1.0-transaction2x1",
            "zokrates-1.0-transaction2x2",
            "zokrates-2.0-rollup1"
        ]
    );
    assert_eq!(config.sequencer().unwrap().host(), "example.com");
    assert_eq!(config.sequencer().unwrap().port(), Some(23433_u16));
    assert!(config.sequencer().unwrap().is_ssl());

    assert_eq!(
        config.packer().unwrap().url(),
        "https://static.mystiko.network/packer/v1"
    );
    assert_eq!(
        config
            .transaction_url(
                5,
                "0xbce8d733536ee3b769456cf91bebae1e9e5be6cb89bb7490c6225384e1bc5e3e"
            )
            .unwrap(),
        "https://goerli.etherscan.io/tx/\
        0xbce8d733536ee3b769456cf91bebae1e9e5be6cb89bb7490c6225384e1bc5e3e"
    );
    assert!(config.transaction_url(234243, "xxx").is_none());
    assert_eq!(config.country_blacklist(), vec!["US", "CN"]);

    assert_eq!(config.screening().unwrap().url(), "https://screening.mystiko.network");
    assert_eq!(config.screening().unwrap().version(), 1);
}

#[tokio::test]
async fn test_selectors() {
    let config = MystikoConfig::from_raw(create_raw_config(true).await).unwrap();
    assert_eq!(
        config
            .find_default_circuit(&CircuitType::Rollup1)
            .unwrap()
            .name(),
        "zokrates-1.0-rollup1"
    );
    assert!(config.find_circuit("zokrates-2.0-rollup1").is_some());
    assert!(config.find_circuit("zokrates-3.0-rollup1").is_none());
    assert!(config.find_chain(3829375345).is_none());
    assert_eq!(config.find_chain(5).unwrap().name(), "Ethereum Goerli");
    let mut peer_chain_ids: Vec<u64> = config
        .find_peer_chains(5)
        .into_iter()
        .map(|c| c.chain_id())
        .collect();
    peer_chain_ids.sort();
    assert_eq!(peer_chain_ids, vec![5, 97]);
    assert!(config.find_peer_chains(24355).is_empty());
    let mut asset_symbols = config.find_asset_symbols(5, 5);
    asset_symbols.sort();
    assert_eq!(asset_symbols, vec!["ETH"]);
    asset_symbols = config.find_asset_symbols(5, 97);
    asset_symbols.sort();
    assert_eq!(asset_symbols, vec!["MTT"]);
    assert!(config.find_asset_symbols(97, 97).is_empty());
    assert!(config.find_asset_symbols(5, 2343534).is_empty());
    let mut bridges: Vec<&BridgeType> = config.find_bridges(5, 97, "MTT");
    bridges.sort_by_key(|b| format!("{:?}", b));
    assert_eq!(
        bridges,
        [
            &BridgeType::Axelar,
            &BridgeType::Celer,
            &BridgeType::LayerZero,
            &BridgeType::Tbridge
        ]
    );
    bridges = config.find_bridges(5, 5, "ETH");
    assert_eq!(bridges, [&BridgeType::Loop]);
    assert!(config.find_bridges(5, 5, "MTT").is_empty());
    assert!(config.find_bridges(97, 24323432, "MTT").is_empty());
    assert_eq!(
        config
            .find_deposit_contract(5, 97, "MTT", &BridgeType::Celer)
            .unwrap()
            .address(),
        "0xe6394a06905d83B19Dbd51804Ca84677a2054FA6"
    );
    assert_eq!(
        config
            .find_deposit_contract(5, 97, "MTT", &BridgeType::Tbridge)
            .unwrap()
            .address(),
        "0xbF5605f5Ed6d18ed957cBA80dbA8838dFcb9A69f"
    );
    assert_eq!(
        config
            .find_deposit_contract(97, 5, "MTT", &BridgeType::Celer)
            .unwrap()
            .address(),
        "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D"
    );
    assert_eq!(
        config
            .find_deposit_contract(97, 5, "MTT", &BridgeType::Tbridge)
            .unwrap()
            .address(),
        "0x9C33eaCc2F50E39940D3AfaF2c7B8246B681A374"
    );
    assert_eq!(
        config
            .find_deposit_contract(5, 5, "ETH", &BridgeType::Loop)
            .unwrap()
            .address(),
        "0x390d485f4d43212d4ae8cdd967a711514ed5a54f"
    );
    assert!(config
        .find_deposit_contract(5, 1024234, "MTT", &BridgeType::Celer)
        .is_none());
    assert!(config
        .find_deposit_contract(1024234, 97, "MTT", &BridgeType::Celer)
        .is_none());
    assert!(config
        .find_deposit_contract_by_address(2342343, "0x9C33eaCc2F50E39940D3AfaF2c7B8246B681A374")
        .is_none());
    assert!(config
        .find_deposit_contract_by_address(5, "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67")
        .is_some());
    assert!(config
        .find_deposit_contract_by_address(97, "0xd791049D0a154bC7860804e1A18ACD148Eb0afD9")
        .is_some());
    assert!(config
        .find_contract_by_address(2342343, "0x9C33eaCc2F50E39940D3AfaF2c7B8246B681A374")
        .is_none());
    assert!(config
        .find_contract_by_address(5, "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67")
        .is_some());
    assert!(config
        .find_contract_by_address(97, "0xd791049D0a154bC7860804e1A18ACD148Eb0afD9")
        .is_some());
    assert_eq!(
        config
            .find_pool_contract(5, "MTT", &BridgeType::Celer, 2)
            .unwrap()
            .address(),
        "0x20Eb345870059E688c59e89523442ade33C7c813"
    );
    assert_eq!(
        config
            .find_pool_contract(5, "MTT", &BridgeType::Tbridge, 2)
            .unwrap()
            .address(),
        "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"
    );
    assert_eq!(
        config
            .find_pool_contract(5, "MTT", &BridgeType::Tbridge, 1)
            .unwrap()
            .address(),
        "0x9b42ec45f6fb6c7d252c66741e960585888de7b6"
    );
    assert_eq!(
        config
            .find_pool_contract(97, "MTT", &BridgeType::Celer, 2)
            .unwrap()
            .address(),
        "0x6B8a4ea37C72F1992626eb9bD48d4aA6aa077c47"
    );
    assert_eq!(
        config
            .find_pool_contract(97, "MTT", &BridgeType::Tbridge, 2)
            .unwrap()
            .address(),
        "0xBe2C9c8a00951662dF3a978b25F448968F0595AE"
    );
    assert_eq!(
        config
            .find_pool_contract(5, "ETH", &BridgeType::Loop, 2)
            .unwrap()
            .address(),
        "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
    );
    assert!(config
        .find_pool_contract(1234234, "MTT", &BridgeType::Celer, 2)
        .is_none());
    assert!(config
        .find_pool_contract_by_address(5, "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")
        .is_some());
    assert!(config
        .find_pool_contract_by_address(2342342, "0xBe2C9c8a00951662dF3a978b25F448968F0595AE")
        .is_none());
    assert!(config
        .find_contract_by_address(5, "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")
        .is_some());
    assert!(config
        .find_contract_by_address(2342342, "0xBe2C9c8a00951662dF3a978b25F448968F0595AE")
        .is_none());
    let mut pool_contract_address: Vec<&str> = config
        .find_pool_contracts(5, "MTT", &BridgeType::Celer)
        .into_iter()
        .map(|c| c.address())
        .collect();
    assert_eq!(
        pool_contract_address,
        vec!["0x20Eb345870059E688c59e89523442ade33C7c813"]
    );
    pool_contract_address = config
        .find_pool_contracts(5, "MTT", &BridgeType::Tbridge)
        .into_iter()
        .map(|c| c.address())
        .collect();
    pool_contract_address.sort();
    assert_eq!(
        pool_contract_address,
        vec![
            "0x9b42ec45f6fb6c7d252c66741e960585888de7b6",
            "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"
        ]
    );
    pool_contract_address = config
        .find_pool_contracts(5, "ETH", &BridgeType::Loop)
        .into_iter()
        .map(|c| c.address())
        .collect();
    assert_eq!(
        pool_contract_address,
        vec!["0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"]
    );
    pool_contract_address = config
        .find_pool_contracts(97, "MTT", &BridgeType::Celer)
        .into_iter()
        .map(|c| c.address())
        .collect();
    assert_eq!(
        pool_contract_address,
        vec!["0x6B8a4ea37C72F1992626eb9bD48d4aA6aa077c47"]
    );
    pool_contract_address = config
        .find_pool_contracts(97, "MTT", &BridgeType::Tbridge)
        .into_iter()
        .map(|c| c.address())
        .collect();
    assert_eq!(
        pool_contract_address,
        vec!["0xBe2C9c8a00951662dF3a978b25F448968F0595AE"]
    );
    assert!(config
        .find_pool_contracts(234234, "MTT", &BridgeType::Celer)
        .is_empty());
}

#[tokio::test]
async fn test_create_from_json_str() {
    assert!(MystikoConfig::from_json_str("{}").is_err());
    assert!(MystikoConfig::from_json_str("{ \"version\": \"0.1.0\"}").is_ok());
}

#[tokio::test]
async fn test_create_from_remote() {
    let mut server = Server::new_async().await;
    let path1 = server
        .mock("GET", "/config/production/mainnet/latest.json")
        .with_body("{\"version\": \"0.2.0\"}")
        .create_async()
        .await;
    let path2 = server
        .mock("GET", "/config/production/mainnet/b5214a4/config.json")
        .with_body("{\"version\": \"0.3.0\"}")
        .create_async()
        .await;
    let path3 = server
        .mock("GET", "/config/production/testnet/latest.json")
        .with_body("{\"version\": \"0.4.0\"}")
        .create_async()
        .await;
    let path4 = server
        .mock("GET", "/config/production/testnet/b5214a4/config.json")
        .with_body("{\"version\": \"0.5.0\"}")
        .create_async()
        .await;
    let path5 = server
        .mock("GET", "/config/staging/mainnet/latest.json")
        .with_body("{\"version\": \"0.6.0\"}")
        .create_async()
        .await;
    let path6 = server
        .mock("GET", "/config/staging/mainnet/b5214a4/config.json")
        .with_body("{\"version\": \"0.7.0\"}")
        .create_async()
        .await;
    let path7 = server
        .mock("GET", "/config/staging/testnet/latest.json")
        .with_body("{\"version\": \"0.8.0\"}")
        .create_async()
        .await;
    let path8 = server
        .mock("GET", "/config/staging/testnet/b5214a4/config.json")
        .with_body("{\"version\": \"0.9.0\"}")
        .create_async()
        .await;
    let base_url = format!("{}/config", server.url());
    let options1 = MystikoConfigOptions::builder()
        .remote_base_url(base_url.clone())
        .build();
    let options2 = MystikoConfigOptions::builder()
        .remote_base_url(base_url.clone())
        .git_revision(String::from("b5214a4"))
        .build();
    let options3 = MystikoConfigOptions::builder()
        .remote_base_url(base_url.clone())
        .is_testnet(true)
        .build();
    let options4 = MystikoConfigOptions::builder()
        .remote_base_url(base_url.clone())
        .git_revision(String::from("b5214a4"))
        .is_testnet(true)
        .build();
    let options5 = MystikoConfigOptions::builder()
        .remote_base_url(base_url.clone())
        .is_staging(true)
        .build();
    let options6 = MystikoConfigOptions::builder()
        .remote_base_url(base_url.clone())
        .is_staging(true)
        .git_revision(String::from("b5214a4"))
        .build();
    let options7 = MystikoConfigOptions::builder()
        .remote_base_url(base_url.clone())
        .is_staging(true)
        .is_testnet(true)
        .build();
    let options8 = MystikoConfigOptions::builder()
        .remote_base_url(base_url.clone())
        .is_staging(true)
        .git_revision(String::from("b5214a4"))
        .is_testnet(true)
        .build();
    let config1 = MystikoConfig::from_remote(&options1).await.unwrap();
    let config2 = MystikoConfig::from_remote(&options2).await.unwrap();
    let config3 = MystikoConfig::from_remote(&options3).await.unwrap();
    let config4 = MystikoConfig::from_remote(&options4).await.unwrap();
    let config5 = MystikoConfig::from_remote(&options5).await.unwrap();
    let config6 = MystikoConfig::from_remote(&options6).await.unwrap();
    let config7 = MystikoConfig::from_remote(&options7).await.unwrap();
    let config8 = MystikoConfig::from_remote(&options8).await.unwrap();
    path1.assert_async().await;
    path2.assert_async().await;
    path3.assert_async().await;
    path4.assert_async().await;
    path5.assert_async().await;
    path6.assert_async().await;
    path7.assert_async().await;
    path8.assert_async().await;
    assert_eq!(config1.version(), "0.2.0");
    assert_eq!(config2.version(), "0.3.0");
    assert_eq!(config3.version(), "0.4.0");
    assert_eq!(config4.version(), "0.5.0");
    assert_eq!(config5.version(), "0.6.0");
    assert_eq!(config6.version(), "0.7.0");
    assert_eq!(config7.version(), "0.8.0");
    assert_eq!(config8.version(), "0.9.0");
}

#[tokio::test]
async fn test_create_from_remote_error() {
    let server = Server::new_async().await;
    let options = MystikoConfigOptions::builder()
        .remote_base_url(format!("{}/config", server.url()))
        .build();
    assert!(MystikoConfig::from_remote(&options).await.is_err());
}

#[tokio::test]
async fn test_create_from_options() {
    let mut server = Server::new_async().await;
    let path1 = server
        .mock("GET", "/config/production/mainnet/latest.json")
        .with_body("{\"version\": \"0.2.0\"}")
        .create_async()
        .await;
    let base_url = format!("{}/config", server.url());
    let options1 = MystikoConfigOptions::builder()
        .remote_base_url(base_url.clone())
        .build();
    let config1 = MystikoConfig::from_options(options1).await.unwrap();
    path1.assert_async().await;
    assert_eq!(config1.version(), "0.2.0");

    let options2 = MystikoConfigOptions::builder()
        .file_path(VALID_CONFIG_FILE.to_string())
        .build();
    let config2 = MystikoConfig::from_options(options2).await.unwrap();
    config2.validate().unwrap();
    assert_eq!(config2.version(), "0.1.0");
    assert_eq!(config2.git_revision().unwrap(), "b6b5b5b");
}

#[tokio::test]
async fn test_create_from_options_with_remote_error() {
    let _ = env_logger::builder()
        .filter_module("mystiko_config", log::LevelFilter::Info)
        .try_init();
    let server = Server::new_async().await;
    let mut options = MystikoConfigOptions::builder()
        .remote_base_url(format!("{}/config", server.url()))
        .build();
    MystikoConfig::from_options(options.clone()).await.unwrap();
    options.is_testnet = true;
    MystikoConfig::from_options(options).await.unwrap();
}

#[test]
fn test_from_local_defaults() {
    MystikoConfig::from_local_default_testnet().unwrap();
    MystikoConfig::from_local_default_mainnet().unwrap();
}

#[tokio::test]
async fn test_duplicate_default_circuit_type() {
    let mut raw_config = create_raw_config(false).await;
    let mut circuit_config = raw_config.circuits.first().unwrap().as_ref().clone();
    circuit_config.is_default = true;
    circuit_config.name = String::from("duplicate default circuit type");
    raw_config.circuits.push(Arc::new(circuit_config));
    assert_eq!(
        MystikoConfig::from_raw(raw_config)
            .err()
            .unwrap()
            .to_string(),
        "duplicate default circuit config for circuit_type Rollup1"
    );
}

#[tokio::test]
async fn test_duplicate_circuit_name() {
    let mut raw_config = create_raw_config(false).await;
    let mut circuit_config = raw_config.circuits.first().unwrap().as_ref().clone();
    circuit_config.is_default = false;
    raw_config.circuits.push(Arc::new(circuit_config));
    assert_eq!(
        MystikoConfig::from_raw(raw_config)
            .err()
            .unwrap()
            .to_string(),
        "duplicate circuit config for name zokrates-1.0-rollup1"
    );
}

#[tokio::test]
async fn test_validate_non_existing_bridge_config() {
    let mut raw_config = create_raw_config(false).await;
    raw_config.bridges.remove(1);
    assert_eq!(
        MystikoConfig::from_raw(raw_config)
            .err()
            .unwrap()
            .to_string(),
        "no bridge config for bridge_type Tbridge"
    );
}

#[tokio::test]
async fn test_validate_peer_bridge_type_mismatch() {
    let mut raw_config = create_raw_config(false).await;
    let mut chain_config = raw_config.chains.remove(1).as_ref().clone();
    let mut deposit_contract_config = chain_config.deposit_contracts.remove(0).as_ref().clone();
    let mut pool_contract_config = chain_config.pool_contracts.remove(0).as_ref().clone();
    deposit_contract_config.bridge_type = BridgeType::Axelar;
    pool_contract_config.bridge_type = BridgeType::Axelar;
    chain_config
        .deposit_contracts
        .push(Arc::new(deposit_contract_config));
    chain_config
        .pool_contracts
        .push(Arc::new(pool_contract_config));
    raw_config.chains.push(Arc::new(chain_config));
    assert_eq!(
        MystikoConfig::from_raw(raw_config)
            .err()
            .unwrap()
            .to_string(),
        "mismatched bridge_types Axelar vs Tbridge for peer deposit contract config \
        of chain_id 97 at 0xd791049D0a154bC7860804e1A18ACD148Eb0afD9 \
        for deposit contract config of chain_id 5 \
        at 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
    );
}

#[tokio::test]
async fn test_validate_peer_chain_id_mismatch() {
    let mut raw_config = create_raw_config(false).await;
    let mut chain_config = raw_config.chains.remove(1).as_ref().clone();
    let mut deposit_contract_config = chain_config.deposit_contracts.remove(0).as_ref().clone();
    deposit_contract_config.peer_chain_id = Some(23423);
    chain_config
        .deposit_contracts
        .push(Arc::new(deposit_contract_config));
    raw_config.chains.push(Arc::new(chain_config));
    assert_eq!(
        MystikoConfig::from_raw(raw_config)
            .err()
            .unwrap()
            .to_string(),
        "peer_chain_id for peer deposit contract config \
        of chain_id 97 at 0xd791049D0a154bC7860804e1A18ACD148Eb0afD9 should be 5"
    );
}

#[tokio::test]
async fn test_validate_peer_contract_address_mismatch() {
    let mut raw_config = create_raw_config(false).await;
    let mut chain_config = raw_config.chains.remove(1).as_ref().clone();
    let mut deposit_contract_config = chain_config.deposit_contracts.remove(0).as_ref().clone();
    deposit_contract_config.peer_contract_address =
        Some(String::from("0xdef9507710e6f69352ea9bcc4bb68f45c15550d9"));
    chain_config
        .deposit_contracts
        .push(Arc::new(deposit_contract_config));
    raw_config.chains.push(Arc::new(chain_config));
    assert_eq!(
        MystikoConfig::from_raw(raw_config)
            .err()
            .unwrap()
            .to_string(),
        "peer_contract_address for peer deposit contract config \
        of chain_id 97 at 0xd791049D0a154bC7860804e1A18ACD148Eb0afD9 \
        should be 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
    );
}

#[tokio::test]
async fn validate_missing_peer_contract_config() {
    let mut raw_config = create_raw_config(false).await;
    let mut chain_config = raw_config.chains.remove(1).as_ref().clone();
    chain_config.deposit_contracts.remove(0);
    raw_config.chains.push(Arc::new(chain_config));
    assert_eq!(
        MystikoConfig::from_raw(raw_config)
            .err()
            .unwrap()
            .to_string(),
        "cannot find peer deposit contract config \
        of chain_id 97 at 0xd791049D0a154bC7860804e1A18ACD148Eb0afD9 \
        for deposit contract config of chain_id 5 \
        at 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
    );
}

#[tokio::test]
async fn validate_missing_peer_chain_config() {
    let mut raw_config = create_raw_config(false).await;
    raw_config.chains.remove(1);
    assert_eq!(
        MystikoConfig::from_raw(raw_config)
            .err()
            .unwrap()
            .to_string(),
        "cannot find chain config of \
        peer_chain_id 97 for deposit contract config \
        at 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 chain_id 5"
    );
}

#[tokio::test]
async fn test_screening_miss_url_config() {
    let config = MystikoConfig::from_raw(create_raw_config(true).await).unwrap();
    assert_eq!(config.screening().unwrap().url(), "https://screening.mystiko.network");
}

async fn create_raw_config(full_config: bool) -> RawMystikoConfig {
    if full_config {
        create_raw_from_file::<RawMystikoConfig>(FULL_CONFIG_FILE)
            .await
            .unwrap()
    } else {
        create_raw_from_file::<RawMystikoConfig>(VALID_CONFIG_FILE)
            .await
            .unwrap()
    }
}
