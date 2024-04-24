use mystiko_relayer_config::raw::gas_cost::RawGasCostConfig;
use mystiko_relayer_config::raw::{create_raw, create_raw_from_file, create_raw_from_json};
use validator::Validate;

fn default_config() -> RawGasCostConfig {
    create_raw::<RawGasCostConfig>(
        RawGasCostConfig::builder()
            .transaction1x0(500704)
            .transaction1x1(619966)
            .transaction1x2(705128)
            .transaction2x0(598799)
            .transaction2x1(708389)
            .transaction2x2(803183)
            .build(),
    )
    .unwrap()
}

#[test]
fn test_valid_config() {
    let config = default_config();
    assert!(config.validate().is_ok());
}

#[test]
fn test_invalid_transaction1x0() {
    let mut config = default_config();
    config.transaction1x0 = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_transaction1x1() {
    let mut config = default_config();
    config.transaction1x1 = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_transaction1x2() {
    let mut config = default_config();
    config.transaction1x2 = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_transaction2x0() {
    let mut config = default_config();
    config.transaction2x0 = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_transaction2x1() {
    let mut config = default_config();
    config.transaction2x1 = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_transaction2x2() {
    let mut config = default_config();
    config.transaction2x2 = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_create_from_json() {
    let json_str = r#"
    {
      "transaction1x0": 500704,
      "transaction1x1": 619966,
      "transaction1x2": 705128,
      "transaction2x0": 598799,
      "transaction2x1": 708389,
      "transaction2x2": 803183
    }
    "#;
    let config = create_raw_from_json::<RawGasCostConfig>(json_str).unwrap();
    assert_eq!(config, default_config());
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config = create_raw_from_file::<RawGasCostConfig>("tests/files/gas_cost.valid.json")
        .await
        .unwrap();
    assert_eq!(file_config, default_config());
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config =
        create_raw_from_file::<RawGasCostConfig>("tests/files/gas_cost.invalid.json").await;
    assert!(file_config.is_err());
}
