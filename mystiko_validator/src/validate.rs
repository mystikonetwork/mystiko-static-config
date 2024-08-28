use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::Arc;
use validator::{Validate, ValidationError};

lazy_static! {
    static ref ETHEREUM_ADDRESS_REGEX: Regex = Regex::new(r"^(0x)[0-9a-fA-F]{40}$").unwrap();
    static ref IS_SEM_VER: Regex = Regex::new(
        r"^(\d+)\.(\d+)\.(\d+)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?$"
    )
    .unwrap();
    static ref NO_SYMBOL_NUMERIC: Regex = Regex::new(r"^[0-9]+$").unwrap();
    static ref NUMERIC_WITH_SYMBOL: Regex = Regex::new(r"^[+-]?([0-9]*[.])?[0-9]+$").unwrap();
    static ref IS_GIT_REVISION: Regex = Regex::new(r"\b[0-9a-f]{7,40}\b").unwrap();
    static ref IS_API_VERSION: Regex = Regex::new(r"^v\d+$").unwrap();
}

pub fn is_ethereum_address(address: &str) -> Result<(), ValidationError> {
    if ETHEREUM_ADDRESS_REGEX.is_match(address) {
        return Ok(());
    }
    Err(ValidationError::new("ethereum address error"))
}

pub fn array_unique<T>(array: &[T]) -> Result<(), ValidationError>
where
    T: Hash + PartialEq + Eq,
{
    let mut seen = HashSet::new();
    for item in array {
        if seen.contains(item) {
            return Err(ValidationError::new("array is not unique"));
        }
        seen.insert(item);
    }
    Ok(())
}

pub fn is_number_string<const NO_SYMBOLS: bool>(s: &str) -> Result<(), ValidationError> {
    if !is_numeric(s, NO_SYMBOLS) {
        return Err(ValidationError::new("is number string error"));
    }
    Ok(())
}

pub fn is_number_string_vec<const NO_SYMBOLS: bool>(v: &[String]) -> Result<(), ValidationError> {
    let is_number = v.iter().all(|s| is_numeric(s, NO_SYMBOLS));
    if !is_number {
        return Err(ValidationError::new("is number string error"));
    }
    Ok(())
}

pub fn validate_nested_vec<T>(v: &[Arc<T>]) -> Result<(), ValidationError>
where
    T: Validate,
{
    if v.iter().all(|x| x.validate().is_ok()) {
        Ok(())
    } else {
        Err(ValidationError::new("validate nested vec error"))
    }
}

pub fn string_vec_each_not_empty(v: &[String]) -> Result<(), ValidationError> {
    if v.iter().all(|s| !s.is_empty()) {
        Ok(())
    } else {
        Err(ValidationError::new("vec element cannot be empty"))
    }
}

pub fn is_sem_ver(s: &str) -> Result<(), ValidationError> {
    if IS_SEM_VER.is_match(s) {
        return Ok(());
    }
    Err(ValidationError::new("version is invalid"))
}

pub fn is_git_revision(s: &str) -> Result<(), ValidationError> {
    if IS_GIT_REVISION.is_match(s) {
        return Ok(());
    }
    Err(ValidationError::new("git revision is invalid"))
}

pub fn is_numeric(s: &str, no_symbol: bool) -> bool {
    if no_symbol {
        NO_SYMBOL_NUMERIC.is_match(s)
    } else {
        NUMERIC_WITH_SYMBOL.is_match(s)
    }
}

pub fn is_api_version(map: &HashMap<u16, String>) -> Result<(), ValidationError> {
    for version in map.values() {
        if !IS_API_VERSION.is_match(version) {
            return Err(ValidationError::new("api version is invalid"));
        }
    }
    Ok(())
}

pub fn is_url_or_empty(s: &str) -> Result<(), ValidationError> {
    if s.is_empty() {
        return Ok(());
    }
    if url::Url::parse(s).is_ok() {
        return Ok(());
    }
    Err(ValidationError::new("url is invalid"))
}
