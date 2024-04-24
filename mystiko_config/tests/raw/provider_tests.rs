use mystiko_config::{create_raw, create_raw_from_file, RawProviderConfig};
use validator::Validate;

fn default_config() -> RawProviderConfig {
    create_raw::<RawProviderConfig>(
        RawProviderConfig::builder()
            .url("http://localhost:8545".to_string())
            .timeout_ms(100000)
            .max_try_count(5)
            .quorum_weight(2)
            .build(),
    )
    .unwrap()
}

#[test]
fn test_invalid_url_0() {
    let mut config = default_config();
    config.url = "".to_string();
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_url_1() {
    let mut config = default_config();
    config.url = "not even a url".to_string();
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_url_2() {
    let mut config = default_config();
    config.url = "wrong_schema://localhost:8545".to_string();
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_timeout_ms() {
    let mut config = default_config();
    config.timeout_ms = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_max_try_count() {
    let mut config = default_config();
    config.max_try_count = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_quorum_weight() {
    let mut config = default_config();
    config.quorum_weight = 0;
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config = create_raw_from_file::<RawProviderConfig>("tests/files/provider/valid.json")
        .await
        .unwrap();
    assert_eq!(file_config, default_config());
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config =
        create_raw_from_file::<RawProviderConfig>("tests/files/provider/invalid.json").await;
    assert!(file_config.is_err());
}
