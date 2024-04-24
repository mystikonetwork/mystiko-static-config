use mystiko_config::{create_raw, create_raw_from_file, RawCircuitConfig};
use mystiko_types::CircuitType;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use validator::Validate;

fn default_config() -> RawCircuitConfig {
    create_raw::<RawCircuitConfig>(
        RawCircuitConfig::builder()
            .name("zokrates-1.0-rollup1".to_string())
            .circuit_type(CircuitType::Rollup1)
            .is_default(true)
            .program_file(vec![String::from("./Rollup1.program.gz")])
            .abi_file(vec![String::from("./Rollup1.abi.json")])
            .proving_key_file(vec![String::from("./Rollup1.pkey.gz")])
            .verifying_key_file(vec![String::from("./Rollup1.vkey.gz")])
            .build(),
    )
    .unwrap()
}

#[test]
fn test_hash() {
    let config1 = default_config();
    let mut hasher = DefaultHasher::new();
    config1.hash(&mut hasher);
    let hash1 = hasher.finish();

    hasher = DefaultHasher::new();
    let mut config2 = default_config();
    config2.name = "zokrates-1.1-rollup1".to_string();
    config2.hash(&mut hasher);
    let hash2 = hasher.finish();

    assert_ne!(hash1, hash2);
}

#[test]
fn test_invalid_name() {
    let mut config = default_config();
    config.name = String::from("");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_program_file() {
    let mut config = default_config();
    config.program_file = vec![String::from("")];
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_abi_file() {
    let mut config = default_config();
    config.abi_file = vec![String::from("")];
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_proving_key_file() {
    let mut config = default_config();
    config.proving_key_file = vec![String::from("")];
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_verifying_key_file() {
    let mut config = default_config();
    config.verifying_key_file = vec![String::from("")];
    assert!(config.validate().is_err());
}

#[test]
fn test_circuit_type_all() {
    assert!(!CircuitType::all().is_empty())
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config = create_raw_from_file::<RawCircuitConfig>("tests/files/circuit/valid.json")
        .await
        .unwrap();
    assert_eq!(file_config, default_config());
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config =
        create_raw_from_file::<RawCircuitConfig>("tests/files/circuit/invalid.json").await;
    assert!(file_config.is_err());
}
