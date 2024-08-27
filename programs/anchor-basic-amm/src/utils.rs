use anchor_lang::prelude::*;
use spl_math::uint::U256;

use crate::error::AmmErrorCode;

pub fn k_from_xy(x: u64, y: u64) -> Result<U256> {
    Ok(U256::from(x)
        .checked_mul(U256::from(y))
        .ok_or(AmmErrorCode::Overflow)?)
}

// Calculate new value of X after depositing Y
// When we swap amount A of Y for X, we must calculate the new balance of X from invariant K
// Y₂ = Y₁ + Amount
// X₂ = K / Y₂
pub fn x2_from_y_swap_amount(x: u64, y: u64, a: u64) -> Result<u64> {
    let k = k_from_xy(x, y)?;
    let x_new = U256::from(y)
        .checked_add(U256::from(a))
        .ok_or(AmmErrorCode::Overflow)?;
    Ok(k.checked_div(U256::from(x_new))
        .ok_or(AmmErrorCode::Overflow)?
        .as_u64())
}

// Calculate new value of Y₂ after depositing X
// When we swap amount A of X for Y, we must calculate the new balance of Y from invariant K
// X₂ = X₁ + Amount
// Y₂ = K / X₂
pub fn y2_from_x_swap_amount(x: u64, y: u64, a: u64) -> Result<u64> {
    x2_from_y_swap_amount(y, x, a)
}

// Calculate the withdraw amount of X from swapping in Y
// ΔX = X₁ - X₂
pub fn delta_x_from_y_swap_amount(x: u64, y: u64, a: u64) -> Result<u64> {
    Ok(x.checked_sub(x2_from_y_swap_amount(x, y, a)?)
        .ok_or(AmmErrorCode::Overflow)?)
}

// Calculate difference in Y from swapping in X
// ΔY = Y₁ - Y₂
pub fn delta_y_from_x_swap_amount(x: u64, y: u64, a: u64) -> Result<u64> {
    delta_x_from_y_swap_amount(y, x, a)
}

// Calculate the amount of X must deposit to get a of Y
// dx = X. dy / Y
pub fn calculate_desired_amount_deposit(x: u64, y: u64, a: u64) -> Result<u64> {
    Ok(U256::from(x)
        .checked_mul(U256::from(a))
        .ok_or(AmmErrorCode::Overflow)?
        .checked_div(U256::from(y))
        .ok_or(AmmErrorCode::Overflow)?
        .as_u64())
}

// Calculate the desired amount of Y to withdraw a of X
// dy = Y.dx / (X + dx)
pub fn calculate_desired_amount_withdraw(x: u64, y: u64, a: u64) -> Result<u64> {
    Ok(U256::from(y)
        .checked_mul(U256::from(a))
        .ok_or(AmmErrorCode::Overflow)?
        .checked_div(
            U256::from(x)
                .checked_add(U256::from(a))
                .ok_or(AmmErrorCode::Overflow)?,
        )
        .ok_or(AmmErrorCode::Overflow)?
        .as_u64())
}
