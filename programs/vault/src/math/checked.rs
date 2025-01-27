#![allow(dead_code)]
use anchor_lang::prelude::*;
use std::fmt::Display;

use crate::error::VaultError;

pub fn checked_add<T>(arg1: T, arg2: T) -> Result<T>
where
    T: num_traits::PrimInt + Display,
{
    if let Some(res) = arg1.checked_add(&arg2) {
        Ok(res)
    } else {
        msg!("Error: Overflow in {} + {}", arg1, arg2);
        err!(VaultError::MathOverflow)
    }
}

pub fn checked_sub<T>(arg1: T, arg2: T) -> Result<T>
where
    T: num_traits::PrimInt + Display,
{
    if let Some(res) = arg1.checked_sub(&arg2) {
        Ok(res)
    } else {
        msg!("Error: Overflow in {} - {}", arg1, arg2);
        err!(VaultError::MathOverflow)
    }
}

pub fn checked_div<T>(arg1: T, arg2: T) -> Result<T>
where
    T: num_traits::PrimInt + Display,
{
    if let Some(res) = arg1.checked_div(&arg2) {
        Ok(res)
    } else {
        msg!("Error: Overflow in {} / {}", arg1, arg2);
        err!(VaultError::MathOverflow)
    }
}

pub fn checked_float_div<T>(arg1: T, arg2: T) -> Result<T>
where
    T: num_traits::Float + Display,
{
    if arg2 == T::zero() {
        msg!("Error: Overflow in {} / {}", arg1, arg2);
        return err!(VaultError::MathOverflow);
    }
    let res = arg1 / arg2;
    if !res.is_finite() {
        msg!("Error: Overflow in {} / {}", arg1, arg2);
        err!(VaultError::MathOverflow)
    } else {
        Ok(res)
    }
}

pub fn checked_ceil_div<T>(arg1: T, arg2: T) -> Result<T>
where
    T: num_traits::PrimInt + Display,
{
    if arg1 > T::zero() {
        if arg1 == arg2 && arg2 != T::zero() {
            return Ok(T::one());
        }
        if let Some(res) = (arg1 - T::one()).checked_div(&arg2) {
            Ok(res + T::one())
        } else {
            msg!("Error: Overflow in {} / {}", arg1, arg2);
            err!(VaultError::MathOverflow)
        }
    } else if let Some(res) = arg1.checked_div(&arg2) {
        Ok(res)
    } else {
        msg!("Error: Overflow in {} / {}", arg1, arg2);
        err!(VaultError::MathOverflow)
    }
}

pub fn checked_decimal_div(
    coefficient1: u64,
    exponent1: i32,
    coefficient2: u64,
    exponent2: i32,
    target_exponent: i32,
) -> Result<u64> {
    // compute scale factor for the dividend
    let mut scale_factor = 0;
    let mut target_power = checked_sub(checked_sub(exponent1, exponent2)?, target_exponent)?;
    if exponent1 > 0 {
        scale_factor = checked_add(scale_factor, exponent1)?;
    }
    if exponent2 < 0 {
        scale_factor = checked_sub(scale_factor, exponent2)?;
        target_power = checked_add(target_power, exponent2)?;
    }
    if target_exponent < 0 {
        scale_factor = checked_sub(scale_factor, target_exponent)?;
        target_power = checked_add(target_power, target_exponent)?;
    }
    let scaled_coeff1 = if scale_factor > 0 {
        checked_mul(
            coefficient1 as u128,
            checked_pow(10u128, scale_factor as usize)?,
        )?
    } else {
        coefficient1 as u128
    };

    if target_power >= 0 {
        checked_as_u64(checked_mul(
            checked_div(scaled_coeff1, coefficient2 as u128)?,
            checked_pow(10u128, target_power as usize)?,
        )?)
    } else {
        checked_as_u64(checked_div(
            checked_div(scaled_coeff1, coefficient2 as u128)?,
            checked_pow(10u128, (-target_power) as usize)?,
        )?)
    }
}

pub fn checked_decimal_ceil_div(
    coefficient1: u64,
    exponent1: i32,
    coefficient2: u64,
    exponent2: i32,
    target_exponent: i32,
) -> Result<u64> {
    // compute scale factor for the dividend
    let mut scale_factor = 0;
    let mut target_power = checked_sub(checked_sub(exponent1, exponent2)?, target_exponent)?;
    if exponent1 > 0 {
        scale_factor = checked_add(scale_factor, exponent1)?;
    }
    if exponent2 < 0 {
        scale_factor = checked_sub(scale_factor, exponent2)?;
        target_power = checked_add(target_power, exponent2)?;
    }
    if target_exponent < 0 {
        scale_factor = checked_sub(scale_factor, target_exponent)?;
        target_power = checked_add(target_power, target_exponent)?;
    }
    let scaled_coeff1 = if scale_factor > 0 {
        checked_mul(
            coefficient1 as u128,
            checked_pow(10u128, scale_factor as usize)?,
        )?
    } else {
        coefficient1 as u128
    };

    if target_power >= 0 {
        checked_as_u64(checked_mul(
            checked_ceil_div(scaled_coeff1, coefficient2 as u128)?,
            checked_pow(10u128, target_power as usize)?,
        )?)
    } else {
        checked_as_u64(checked_div(
            checked_ceil_div(scaled_coeff1, coefficient2 as u128)?,
            checked_pow(10u128, (-target_power) as usize)?,
        )?)
    }
}

pub fn checked_token_div(
    amount1: u64,
    decimals1: u8,
    amount2: u64,
    decimals2: u8,
) -> Result<(u64, u8)> {
    let target_decimals = std::cmp::max(decimals1, decimals2);
    Ok((
        checked_decimal_div(
            amount1,
            -(decimals1 as i32),
            amount2,
            -(decimals2 as i32),
            -(target_decimals as i32),
        )?,
        target_decimals,
    ))
}

pub fn checked_mul<T>(arg1: T, arg2: T) -> Result<T>
where
    T: num_traits::PrimInt + Display,
{
    if let Some(res) = arg1.checked_mul(&arg2) {
        Ok(res)
    } else {
        msg!("Error: Overflow in {} * {}", arg1, arg2);
        err!(VaultError::MathOverflow)
    }
}

pub fn checked_float_mul<T>(arg1: T, arg2: T) -> Result<T>
where
    T: num_traits::Float + Display,
{
    let res = arg1 * arg2;
    if !res.is_finite() {
        msg!("Error: Overflow in {} * {}", arg1, arg2);
        err!(VaultError::MathOverflow)
    } else {
        Ok(res)
    }
}

pub fn checked_decimal_mul(
    coefficient1: u64,
    exponent1: i32,
    coefficient2: u64,
    exponent2: i32,
    target_exponent: i32,
) -> Result<u64> {
    let target_power = checked_sub(checked_add(exponent1, exponent2)?, target_exponent)?;
    if target_power >= 0 {
        checked_as_u64(checked_mul(
            checked_mul(coefficient1 as u128, coefficient2 as u128)?,
            checked_pow(10u128, target_power as usize)?,
        )?)
    } else {
        checked_as_u64(checked_div(
            checked_mul(coefficient1 as u128, coefficient2 as u128)?,
            checked_pow(10u128, (-target_power) as usize)?,
        )?)
    }
}

pub fn checked_decimal_ceil_mul(
    coefficient1: u64,
    exponent1: i32,
    coefficient2: u64,
    exponent2: i32,
    target_exponent: i32,
) -> Result<u64> {
    let target_power = checked_sub(checked_add(exponent1, exponent2)?, target_exponent)?;
    if target_power >= 0 {
        checked_as_u64(checked_mul(
            checked_mul(coefficient1 as u128, coefficient2 as u128)?,
            checked_pow(10u128, target_power as usize)?,
        )?)
    } else {
        checked_as_u64(checked_ceil_div(
            checked_mul(coefficient1 as u128, coefficient2 as u128)?,
            checked_pow(10u128, (-target_power) as usize)?,
        )?)
    }
}

pub fn checked_token_mul(
    amount1: u64,
    decimals1: u8,
    amount2: u64,
    decimals2: u8,
) -> Result<(u64, u8)> {
    let target_decimals = std::cmp::max(decimals1, decimals2);
    Ok((
        checked_decimal_mul(
            amount1,
            -(decimals1 as i32),
            amount2,
            -(decimals2 as i32),
            -(target_decimals as i32),
        )?,
        target_decimals,
    ))
}

pub fn checked_pow<T>(arg: T, exp: usize) -> Result<T>
where
    T: num_traits::PrimInt + Display,
{
    if let Some(res) = num_traits::checked_pow(arg, exp) {
        Ok(res)
    } else {
        msg!("Error: Overflow in {} ^ {}", arg, exp);
        err!(VaultError::MathOverflow)
    }
}

pub fn checked_powf(arg: f64, exp: f64) -> Result<f64> {
    let res = f64::powf(arg, exp);
    if res.is_finite() {
        Ok(res)
    } else {
        msg!("Error: Overflow in {} ^ {}", arg, exp);
        err!(VaultError::MathOverflow)
    }
}

pub fn checked_powi(arg: f64, exp: i32) -> Result<f64> {
    let res = if exp > 0 {
        f64::powi(arg, exp)
    } else {
        // wrokaround due to f64::powi() not working properly on-chain with negative exponent
        checked_float_div(1.0, f64::powi(arg, -exp))?
    };
    if res.is_finite() {
        Ok(res)
    } else {
        msg!("Error: Overflow in {} ^ {}", arg, exp);
        err!(VaultError::MathOverflow)
    }
}

pub fn checked_as_u64<T>(arg: T) -> Result<u64>
where
    T: Display + num_traits::ToPrimitive + Clone,
{
    let option: Option<u64> = num_traits::NumCast::from(arg.clone());
    if let Some(res) = option {
        Ok(res)
    } else {
        msg!("Error: Overflow in {} as u64", arg);
        err!(VaultError::MathOverflow)
    }
}

pub fn checked_as_u128<T>(arg: T) -> Result<u128>
where
    T: Display + num_traits::ToPrimitive + Clone,
{
    let option: Option<u128> = num_traits::NumCast::from(arg.clone());
    if let Some(res) = option {
        Ok(res)
    } else {
        msg!("Error: Overflow in {} as u128", arg);
        err!(VaultError::MathOverflow)
    }
}

pub fn to_ui_amount(amount: u64, decimals: u8) -> Result<f64> {
    checked_float_div(amount as f64, checked_powi(10.0, decimals as i32)?)
}

pub fn to_token_amount(ui_amount: f64, decimals: u8) -> Result<u64> {
    checked_as_u64(checked_float_mul(
        ui_amount,
        checked_powi(10.0, decimals as i32)?,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checked_add() {
        assert_eq!(checked_add(2u64, 3u64).unwrap(), 5u64);
        assert_eq!(checked_add(0u64, 0u64).unwrap(), 0u64);
        // Test overflow
        assert!(checked_add(u64::MAX, 1u64).is_err());
    }

    #[test]
    fn test_checked_sub() {
        assert_eq!(checked_sub(5u64, 3u64).unwrap(), 2u64);
        assert_eq!(checked_sub(0u64, 0u64).unwrap(), 0u64);
        // Test underflow
        assert!(checked_sub(0u64, 1u64).is_err());
    }

    #[test]
    fn test_checked_mul() {
        assert_eq!(checked_mul(2u64, 3u64).unwrap(), 6u64);
        assert_eq!(checked_mul(0u64, 5u64).unwrap(), 0u64);
        // Test overflow
        assert!(checked_mul(u64::MAX, 2u64).is_err());
    }

    #[test]
    fn test_checked_div() {
        assert_eq!(checked_div(6u64, 2u64).unwrap(), 3u64);
        assert_eq!(checked_div(0u64, 5u64).unwrap(), 0u64);
        // Test division by zero
        assert!(checked_div(5u64, 0u64).is_err());
    }

    #[test]
    fn test_checked_ceil_div() {
        assert_eq!(checked_ceil_div(5u64, 2u64).unwrap(), 3u64);
        assert_eq!(checked_ceil_div(6u64, 2u64).unwrap(), 3u64);
        assert_eq!(checked_ceil_div(0u64, 5u64).unwrap(), 0u64);
        // Test division by zero
        assert!(checked_ceil_div(5u64, 0u64).is_err());
        // Test equal numbers
        assert_eq!(checked_ceil_div(5u64, 5u64).unwrap(), 1u64);
    }

    #[test]
    fn test_checked_float_div() {
        assert_eq!(checked_float_div(6.0f64, 2.0f64).unwrap(), 3.0f64);
        // Test division by zero
        assert!(checked_float_div(5.0f64, 0.0f64).is_err());
        // Test infinity result
        assert!(checked_float_div(f64::MAX, 0.1f64).is_err());
    }

    #[test]
    fn test_checked_pow() {
        assert_eq!(checked_pow(2u64, 3).unwrap(), 8u64);
        assert_eq!(checked_pow(0u64, 0).unwrap(), 1u64);
        assert_eq!(checked_pow(0u64, 5).unwrap(), 0u64);
        // Test overflow
        assert!(checked_pow(2u64, 64).is_err());
    }

    #[test]
    fn test_checked_decimal_div() {
        // Test basic division
        assert_eq!(
            checked_decimal_div(100, -2, 2, 0, -2).unwrap(),
            50
        );
        // Test with different exponents
        assert_eq!(
            checked_decimal_div(1000, -3, 2, 0, -3).unwrap(),
            500
        );
    }

    #[test]
    fn test_token_conversion() {
        // Test UI amount to token amount conversion
        let ui_amount = 1.5f64;
        let decimals = 6u8;
        let token_amount = to_token_amount(ui_amount, decimals).unwrap();
        assert_eq!(token_amount, 1_500_000);

        // Test token amount to UI amount conversion
        let converted_ui_amount = to_ui_amount(token_amount, decimals).unwrap();
        assert_eq!(converted_ui_amount, ui_amount);

        // Test with zero
        assert_eq!(to_token_amount(0.0, decimals).unwrap(), 0);
        assert_eq!(to_ui_amount(0, decimals).unwrap(), 0.0);

        // Test with max decimals
        let max_decimals = 9u8;
        assert!(to_token_amount(1.0, max_decimals).is_ok());
    }

    #[test]
    fn test_checked_as_u64() {
        assert_eq!(checked_as_u64(5u128).unwrap(), 5u64);
        assert_eq!(checked_as_u64(0u128).unwrap(), 0u64);
        // Test overflow
        assert!(checked_as_u64(u128::MAX).is_err());
    }

    #[test]
    fn test_checked_powi() {
        // Test positive exponent
        assert_eq!(checked_powi(2.0, 3).unwrap(), 8.0);
        // Test negative exponent
        assert_eq!(checked_powi(2.0, -1).unwrap(), 0.5);
        // Test zero base
        assert_eq!(checked_powi(0.0, 3).unwrap(), 0.0);
        // Test overflow
        assert!(checked_powi(f64::MAX, 2).is_err());
    }
}