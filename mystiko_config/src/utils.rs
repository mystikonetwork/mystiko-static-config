use anyhow::Result;
use num_bigint::BigUint;
use num_traits::{zero, NumCast, Zero};
use rust_decimal::Decimal;
use std::fmt::Display;
use std::str::FromStr;

pub const DEFAULT_NUM_DECIMALS: u32 = 18;

pub fn decimal_to_number<T, S>(decimal: &S, num_decimals: Option<u32>) -> Result<T>
where
    S: Display,
    T: NumCast + Zero,
{
    let decimal = Decimal::from_str(&decimal.to_string())?;
    let divisor = Decimal::from_str(
        &BigUint::from(10u32)
            .pow(num_decimals.unwrap_or(DEFAULT_NUM_DECIMALS))
            .to_string(),
    )?;
    Ok(T::from(decimal / divisor).unwrap_or(zero::<T>()))
}
