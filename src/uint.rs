//! Handling of values marked as a 'uint' (`!SIGN_BIT | NAN_BITS`): 52-bit integers.

use super::{cons::*, IntoRawBits64};
use core::num::NonZeroU64;

/// Indicates that the value is a uint; the sign-bit is **not** set.
pub const UINT_MARKER_BITS: u64 = NAN_BITS;

/// Masks the bits ([`UINT_MARKER_BITS`]) that indicate that the value is a uint.
pub const UINT_MARKER_MASK: u64 = SIGN_BIT | NAN_BITS;

/// Masks the bits that actually hold the data of the uint.
pub const UINT_DATA_BITS: u64 = !UINT_MARKER_MASK;

/// Returns wether the given value is a uint.
#[inline(always)]
pub fn is_uint(value: impl IntoRawBits64) -> bool {
    (value.as_raw_bits_64() & (UINT_MARKER_MASK)) == UINT_MARKER_BITS
}

/// Returns wether the given value is *not* a uint.
#[inline(always)]
pub fn is_not_uint(value: impl IntoRawBits64) -> bool {
    (value.as_raw_bits_64() & (UINT_MARKER_MASK)) != UINT_MARKER_BITS
}

/// Unwraps the data of the given value as [`u64`], without checking if it is a uint.
#[inline(always)]
pub fn unwrap_uint_unchecked(value: impl IntoRawBits64) -> u64 {
    value.as_raw_bits_64() & UINT_DATA_BITS
}

/// Unwraps the data of the given value as [`u64`], if it is a uint.
#[inline(always)]
pub fn unwrap_uint(value: impl IntoRawBits64) -> Option<u64> {
    match is_uint(value) {
        true => Some(unwrap_uint_unchecked(value)),
        false => None
    }
}

/// Unwraps the data of the given value as [`NonZeroU64`], if it is a uint.
#[inline(always)]
pub fn unwrap_uint_nonzero(value: impl IntoRawBits64) -> Option<NonZeroU64> {
    match is_uint(value) {
        true => NonZeroU64::new(unwrap_uint_unchecked(value)),
        false => None
    }
}