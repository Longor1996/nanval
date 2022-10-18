#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub mod cons;
pub mod raw;
pub mod uint;

#[cfg(feature = "cell")]
pub mod cell;

pub use raw::IntoRawBits64;
use cons::*;

/// Checks if the given value is a valid `f64`.
/// 
/// There are exactly two cases where this is true:
/// - If the values bits masked with [`NAN_BITS`] are NOT [`NAN_BITS`].
/// - If the values bits EXACTLY match [`NAN_BITS`].
#[inline(always)]
pub fn is_float(value: impl IntoRawBits64) -> bool {
    let value = value.as_raw_bits_64();
    value & NAN_BITS != NAN_BITS || value == NAN_BITS
}

/// Checks if the given value is a NaN-tagged value.
/// 
/// Calls [`is_float`] and negates the result.
#[inline(always)]
pub fn is_nanval(value: impl IntoRawBits64) -> bool {
    ! is_float(value)
}

/// Returns the value as `f64`; does *not* check if the value is actually a float.
#[inline(always)]
pub fn unwrap_float_unchecked(value: impl IntoRawBits64) -> f64 {
    f64::from_bits(value.as_raw_bits_64())
}

/// Returns the value as `f64`, if it is a valid 64-bit floating point number.
#[inline(always)]
pub fn unwrap_float(value: impl IntoRawBits64) -> Option<f64> {
    match is_float(value) {
        true => Some(unwrap_float_unchecked(value)),
        false => None
    }
}
