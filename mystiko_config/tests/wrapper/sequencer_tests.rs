use std::sync::Arc;

use mystiko_config::{create_raw_from_file, RawSequencerConfig, SequencerConfig};

const VALID_CONFIG_FILE: &str = "tests/files/sequencer/full.json";

#[tokio::test]
async fn test_create() {
    let raw_config = create_raw_from_file::<RawSequencerConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = SequencerConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
    assert_eq!(config.host(), "example.com");
    assert_eq!(config.port(), Some(443_u16));
    assert!(config.is_ssl());
}
