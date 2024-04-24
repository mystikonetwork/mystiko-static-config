use mockito::Server;
use mystiko_relayer_config::raw::chain::RawChainConfig;
use mystiko_relayer_config::raw::create_raw_from_file;
use mystiko_relayer_config::raw::relayer::RawRelayerConfig;
use mystiko_relayer_config::wrapper::chain::ChainConfig;
use mystiko_relayer_config::wrapper::relayer::{RelayerConfig, RemoteOptions};
use std::sync::Arc;

const VALID_CONFIG_FILE: &str = "tests/files/relayer.valid.json";
const VALID_CHAIN_CONFIG_FILE: &str = "tests/files/chain.valid.json";

async fn create_chain_config() -> ChainConfig {
    let raw_config = create_raw_from_file::<RawChainConfig>(VALID_CHAIN_CONFIG_FILE)
        .await
        .unwrap();
    let config = ChainConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
    config
}

#[tokio::test]
async fn test_create() {
    let raw_config = create_raw_from_file::<RawRelayerConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = RelayerConfig::from_raw(raw_config).unwrap();
    config.validate().unwrap();

    assert_eq!(config.version(), "0.0.1");
    let chain_config_option = config.find_chain_config(5);
    assert!(chain_config_option.is_some());
    let chain_config = chain_config_option.unwrap();
    assert_eq!(chain_config, &create_chain_config().await);
    assert!(config.find_chain_config(97).is_none());

    assert_eq!(config.chains().len(), 1);
}

#[tokio::test]
#[should_panic(expected = "duplicate default chain config for chain_id 5")]
async fn test_create_invalid_config() {
    let mut raw_config = create_raw_from_file::<RawRelayerConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let raw_chain_config = create_raw_from_file::<RawChainConfig>(VALID_CHAIN_CONFIG_FILE)
        .await
        .unwrap();
    raw_config.chains.push(Arc::new(raw_chain_config));
    let config = RelayerConfig::from_raw(raw_config);
    assert!(config.is_err());
    config.unwrap();
}

#[test]
fn test_create_from_json_str() {
    assert!(RelayerConfig::from_json_str("{}").is_err());
    let json_str = r#"
        {
          "version": "0.0.1",
          "gitRevision": "3f25038",
          "chains": []
        }
    "#;
    assert!(RelayerConfig::from_json_str(json_str).is_ok());
}

#[tokio::test]
async fn test_create_from_json_file() {
    let config = RelayerConfig::from_json_file(VALID_CONFIG_FILE)
        .await
        .unwrap();
    config.validate().unwrap();
}

#[tokio::test]
async fn test_create_from_remote() {
    let mut server = Server::new_async().await;
    let path1 = server
        .mock("GET", "/relayer_config/production/mainnet/latest.json")
        .with_body("{\"version\": \"0.2.0\"}")
        .create_async()
        .await;
    let path2 = server
        .mock(
            "GET",
            "/relayer_config/production/mainnet/b5214a4/config.json",
        )
        .with_body("{\"version\": \"0.3.0\"}")
        .create_async()
        .await;
    let path3 = server
        .mock("GET", "/relayer_config/production/testnet/latest.json")
        .with_body("{\"version\": \"0.4.0\"}")
        .create_async()
        .await;
    let path4 = server
        .mock(
            "GET",
            "/relayer_config/production/testnet/b5214a4/config.json",
        )
        .with_body("{\"version\": \"0.5.0\"}")
        .create_async()
        .await;
    let path5 = server
        .mock("GET", "/relayer_config/staging/mainnet/latest.json")
        .with_body("{\"version\": \"0.6.0\"}")
        .create_async()
        .await;
    let path6 = server
        .mock("GET", "/relayer_config/staging/mainnet/b5214a4/config.json")
        .with_body("{\"version\": \"0.7.0\"}")
        .create_async()
        .await;
    let path7 = server
        .mock("GET", "/relayer_config/staging/testnet/latest.json")
        .with_body("{\"version\": \"0.8.0\"}")
        .create_async()
        .await;
    let path8 = server
        .mock("GET", "/relayer_config/staging/testnet/b5214a4/config.json")
        .with_body("{\"version\": \"0.9.0\"}")
        .create_async()
        .await;
    let base_url = format!("{}/relayer_config", server.url());
    let options1 = RemoteOptions::builder().base_url(base_url.clone()).build();
    let options2 = RemoteOptions::builder()
        .base_url(base_url.clone())
        .git_revision(String::from("b5214a4"))
        .build();
    let options3 = RemoteOptions::builder()
        .base_url(base_url.clone())
        .is_testnet(true)
        .build();
    let options4 = RemoteOptions::builder()
        .base_url(base_url.clone())
        .git_revision(String::from("b5214a4"))
        .is_testnet(true)
        .build();
    let options5 = RemoteOptions::builder()
        .base_url(base_url.clone())
        .is_staging(true)
        .build();
    let options6 = RemoteOptions::builder()
        .base_url(base_url.clone())
        .is_staging(true)
        .git_revision(String::from("b5214a4"))
        .build();
    let options7 = RemoteOptions::builder()
        .base_url(base_url.clone())
        .is_staging(true)
        .is_testnet(true)
        .build();
    let options8 = RemoteOptions::builder()
        .base_url(base_url.clone())
        .is_staging(true)
        .git_revision(String::from("b5214a4"))
        .is_testnet(true)
        .build();
    let config1 = RelayerConfig::from_remote(&options1).await.unwrap();
    let config2 = RelayerConfig::from_remote(&options2).await.unwrap();
    let config3 = RelayerConfig::from_remote(&options3).await.unwrap();
    let config4 = RelayerConfig::from_remote(&options4).await.unwrap();
    let config5 = RelayerConfig::from_remote(&options5).await.unwrap();
    let config6 = RelayerConfig::from_remote(&options6).await.unwrap();
    let config7 = RelayerConfig::from_remote(&options7).await.unwrap();
    let config8 = RelayerConfig::from_remote(&options8).await.unwrap();
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
    let options = RemoteOptions::builder()
        .base_url(format!("{}/relayer_config", server.url()))
        .build();
    assert!(RelayerConfig::from_remote(&options).await.is_err());
}

#[test]
fn test_create_from_local_defaults() {
    RelayerConfig::from_local_default_testnet().unwrap();
    RelayerConfig::from_local_default_mainnet().unwrap();
}
