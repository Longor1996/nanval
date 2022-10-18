//! Various important constants for NaN-tagging, bit-masking, etc.

/// The maximum integer that can be losslessly stored in an [`f64`] as an [`u64`]: `2 ** 52 - 1`
pub const MAX_INT: u64 = 9007199254740991;

/// The maximum integer that can be losslessly stored in an [`f64`] as an [`f64`]: `2 ** 52 - 1`
pub const MAX_INTF: f64 = 9007199254740991.0;

/// If this bit-mask matches, the data bits are a [`crate::cell`].
pub const SIGN_BIT: u64 = 0x8000000000000000;

/// If this bit-mask matches, the value is Not-A-Number / [`f64::NaN`](https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.NAN).
pub const NAN_BITS: u64 = 0x7FF8000000000000;

/// This bit-pattern represents positive infinity / [`f64::INFINITY`](https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.INFINITY).
pub const POS_INF_BITS: u64 = 0x7FF0000000000000;

/// This bit-pattern represents negative infinity / [`f64::NEG_INFINITY`](https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.NEG_INFINITY).
pub const NEG_INF_BITS: u64 = 0xFFF0000000000000;

/// This bit-pattern represents negative zero.
pub const NEG_ZERO_BITS: u64 = 0x8000000000000000;
