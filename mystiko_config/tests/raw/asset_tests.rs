use mystiko_config::{create_raw, create_raw_from_file, RawAssetConfig};
use mystiko_types::AssetType;
use validator::Validate;

fn default_config() -> RawAssetConfig {
    create_raw::<RawAssetConfig>(
        RawAssetConfig::builder()
            .asset_type(AssetType::Erc20)
            .asset_symbol("MTT".to_string())
            .asset_decimals(16)
            .asset_address("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a".to_string())
            .recommended_amounts(vec![
                String::from("10000000000000000"),
                String::from("100000000000000000"),
            ])
            .build(),
    )
    .unwrap()
}

#[test]
fn test_invalid_asset_symbol() {
    let mut config = default_config();
    config.asset_symbol = "".to_string();
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_asset_address_0() {
    let mut config = default_config();
    config.asset_address = String::from("");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_asset_address_1() {
    let mut config = default_config();
    config.asset_address = String::from("0xdeadbeef");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_recommended_amounts_0() {
    let mut config = default_config();
    config.recommended_amounts = vec![String::from("")];
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_recommended_amounts_1() {
    let mut config = default_config();
    config.recommended_amounts = vec![String::from("abcd")];
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_recommended_amounts_2() {
    let mut config = default_config();
    config.recommended_amounts = vec![String::from("1"), String::from("1")];
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config = create_raw_from_file::<RawAssetConfig>("tests/files/asset/valid.json")
        .await
        .unwrap();
    assert_eq!(file_config, default_config());
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config =
        create_raw_from_file::<RawAssetConfig>("tests/files/asset/invalid.json").await;
    assert!(file_config.is_err());
}
