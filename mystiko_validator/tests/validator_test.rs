use mystiko_validator::validate::{
    array_unique, is_api_version, is_ethereum_address, is_number_string, is_number_string_vec,
    is_numeric, is_sem_ver, string_vec_each_not_empty,
};
use std::collections::HashMap;

#[test]
fn test_is_ethereum_address() {
    let mut address = "0x0000000";
    assert!(is_ethereum_address(address).is_err());
    address = "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d";
    assert!(is_ethereum_address(address).is_ok());
}

#[test]
fn test_array_unique() {
    let mut arr = vec!["a", "a", "b", "c"];
    assert!(array_unique::<&str>(&arr).is_err());
    arr = vec!["1", "2", "3"];
    assert!(array_unique::<Vec<&str>>(&[arr]).is_ok());
}

#[test]
fn test_is_number_string() {
    let mut number_vec = vec![
        String::from("10000000000000000"),
        String::from("100000000000000000"),
    ];
    let is_number = is_number_string_vec::<true>(&number_vec);
    assert!(is_number.is_ok());

    number_vec = vec!["abc".to_string()];
    let is_number = is_number_string_vec::<true>(&number_vec);
    assert!(is_number.is_err());

    number_vec = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let is_number = is_number_string_vec::<false>(&number_vec);
    assert!(is_number.is_err());

    assert!(is_number_string::<true>("123").is_ok());
}

#[test]
fn test_string_vec_each_not_empty() {
    let mut v = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let not_empty = string_vec_each_not_empty(&v);
    assert!(not_empty.is_ok());
    v = vec!["a".to_string(), "b".to_string(), "".to_string()];
    let not_empty = string_vec_each_not_empty(&v);
    assert!(not_empty.is_err());
}

#[test]
fn test_is_sem_ver() {
    let mut v = String::from("1.2.3");
    assert!(is_sem_ver(&v).is_ok());
    v = String::from("0.1.0");
    assert!(is_sem_ver(&v).is_ok());
    v = String::from("2.0.0-alpha.1");
    assert!(is_sem_ver(&v).is_ok());
    v = String::from("3.4.5-beta+20181012");
    assert!(is_sem_ver(&v).is_ok());
    v = String::from("0");
    assert!(is_sem_ver(&v).is_err());
}

#[test]
fn test_is_numeric() {
    let mut s = String::from("100");
    assert!(is_numeric(&s, true));
    s = String::from("+100");
    assert!(!is_numeric(&s, true));
    assert!(is_numeric(&s, false));
    s = String::from("-10");
    assert!(!is_numeric(&s, true));
    assert!(is_numeric(&s, false));
    s = String::from("1.2");
    assert!(!is_numeric(&s, true));
}

#[test]
fn test_is_api_version() {
    let mut map = HashMap::new();
    map.insert(0, "v1".to_string());
    map.insert(1, "v2".to_string());
    assert!(is_api_version(&map).is_ok());
    map.insert(2, "v1.0".to_string());
    assert!(is_api_version(&map).is_err());
}
