//! Trait for functions that can accept any sized+copy 64-bit value.
use super::cons::*;

/// Trait for functions that accept any 64-bit value, used throughout this crate.
/// 
/// Since the trait is only `impl`d as the input of functions,
/// monomorphization will cause a N×M relation of structs implementing this trait,
/// with the various functions consuming this trait; at the time of writing this,
/// that would be `5×19`, yielding `95` functions for rustc to emit... which is acceptable.
/// 
/// **Note:** It is *highly* recommended to refer to this traits methods in static-form; ie: `IntoRawBits64::XXX`.
pub trait IntoRawBits64: Copy + Clone + Sized {
    /// Returns the raw bits that `self` contains.
    fn as_raw_bits_64(&self) -> u64;
    
    /// Returns the raw bits that `self` contains, but without the sign-bit.
    #[inline(always)]
    fn as_raw_bits_64_without_sign(&self) -> u64 {
        self.as_raw_bits_64() & !SIGN_BIT
    }
    
    /// Checks if the value is a floating point number.
    #[inline(always)]
    fn is_float(&self) -> bool {
        crate::is_float(*self)
    }
    
    /// Checks if the value is a NaN-tagged value.
    #[inline(always)]
    fn is_nanval(&self) -> bool {
        crate::is_nanval(*self)
    }
    
    /// Checks if the value is a floating point number representing infinity.
    #[inline(always)]
    fn is_nan(&self) -> bool {
        self.as_raw_bits_64() == NAN_BITS
    }
    
    /// Checks if the value is a floating point number representing infinity.
    #[inline(always)]
    fn is_infinity(&self) -> bool {
        self.as_raw_bits_64_without_sign() == POS_INF_BITS
    }
    
    /// Checks if the value is a floating point number representing positive infinity.
    #[inline(always)]
    fn is_positive_infinity(&self) -> bool {
        self.as_raw_bits_64() == POS_INF_BITS
    }
    
    /// Checks if the value is a floating point number representing negative infinity.
    #[inline(always)]
    fn is_negative_infinity(&self) -> bool {
        self.as_raw_bits_64() == NEG_INF_BITS
    }
    
    /// Returns the value as a floating point number, checking if it is one.
    #[inline(always)]
    fn unwrap_float(&self) -> Option<f64> {
        crate::unwrap_float(self.as_raw_bits_64())
    }
    
    /// Returns the value as a floating point number, *without* checking if it is one.
    #[inline(always)]
    fn unwrap_float_unchecked(&self) -> f64 {
        crate::unwrap_float_unchecked(self.as_raw_bits_64())
    }
}

impl IntoRawBits64 for [core::primitive::u8; 8] {
    #[inline(always)]
    fn as_raw_bits_64(&self) -> u64 {
        core::primitive::u64::from_ne_bytes(*self)
    }
}

impl IntoRawBits64 for core::primitive::f64 {
    #[inline(always)]
    fn as_raw_bits_64(&self) -> u64 {
        self.to_bits()
    }
    
    #[inline(always)]
    fn unwrap_float_unchecked(&self) -> f64 {
        *self
    }
}

impl IntoRawBits64 for core::primitive::u64 {
    #[inline(always)]
    fn as_raw_bits_64(&self) -> u64 {
        *self
    }
}

impl IntoRawBits64 for core::num::NonZeroU64 {
    #[inline(always)]
    fn as_raw_bits_64(&self) -> u64 {
        self.get()
    }
}

impl IntoRawBits64 for &[core::primitive::u8] {
    #[inline(always)]
    fn as_raw_bits_64(&self) -> u64 {
        match self.len() >= 8 {
            true => {
                let bytes = self[0..=3].try_into().unwrap();
                core::primitive::u64::from_ne_bytes(bytes)
            },
            false => panic!("not enough bytes to transmute into a u64")
        }
    }
}
