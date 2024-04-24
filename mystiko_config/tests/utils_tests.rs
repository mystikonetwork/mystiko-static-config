use num_bigint::BigUint;
use rust_decimal::Decimal;

use mystiko_config::decimal_to_number;

#[test]
fn test_decimal_to_number() {
    assert_eq!(
        decimal_to_number::<u32, String>(&String::from("1000000000000000000"), None).unwrap(),
        1u32
    );
    assert_eq!(decimal_to_number::<f64, u32>(&1, Some(4)).unwrap(), 0.0001);
    assert_eq!(
        decimal_to_number::<f64, BigUint>(&BigUint::from(10u32), Some(3)).unwrap(),
        0.01
    );
    assert_eq!(
        decimal_to_number::<f32, Decimal>(&Decimal::from(-100), Some(5)).unwrap(),
        -0.001
    );
}
