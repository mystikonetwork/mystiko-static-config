use mystiko_config::{create_raw, create_raw_from_file, RawScreeningConfig};
use validator::Validate;

fn default_config() -> RawScreeningConfig {
    RawScreeningConfig::default()
}

fn full_config() -> RawScreeningConfig {
    create_raw::<RawScreeningConfig>(
        RawScreeningConfig::builder()
            .url("https://screening.mystiko.network")
            .client_timeout_ms(20000u64)
            .version(1u32)
            .build(),
    )
    .unwrap()
}

#[test]
fn test_invalid_url() {
    let mut config = default_config();
    config.url = String::from("not a valid url");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_client_time_out_ms() {
    let mut config = default_config();
    config.client_timeout_ms = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_version() {
    let mut config = default_config();
    config.version = 0;
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config =
        create_raw_from_file::<RawScreeningConfig>("tests/files/screening/valid.json")
            .await
            .unwrap();
    assert_eq!(file_config, default_config());
}

#[tokio::test]
async fn test_import_full_json_file() {
    let file_config = create_raw_from_file::<RawScreeningConfig>("tests/files/screening/full.json")
        .await
        .unwrap();
    assert_eq!(file_config, full_config());
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config =
        create_raw_from_file::<RawScreeningConfig>("tests/files/screening/invalid.json").await;
    assert!(file_config.is_err());
}
