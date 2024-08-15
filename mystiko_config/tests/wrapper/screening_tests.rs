use mystiko_config::{create_raw_from_file, RawScreeningConfig, ScreeningConfig};
use std::sync::Arc;

const VALID_CONFIG_FILE: &str = "tests/files/screening/valid.json";

#[tokio::test]
async fn test_create() {
    let raw_config = create_raw_from_file::<RawScreeningConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = ScreeningConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
    assert_eq!(config.url(), "https://screening.mystiko.network");
    assert_eq!(config.client_timeout_ms(), 20000);
    assert_eq!(config.version(), 1);
}
