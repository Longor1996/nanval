//! Handling of values marked as a 'cell' (`SIGN_BIT | NAN_BITS`, with 3 'tag' bits).
//! 
//! Bit Layout is as follows:
//! ```text
//! s111 1111 1111 1ttt xxxx xxxx xxxx xxxx xxxx xxxx xxxx xxxx xxxx xxxx xxxx xxxx
//! ```
//! 
//! - Sign `s`; always `1` for a cell-value.
//! - Tag  `t`, 3 bits; `0b000` is undefined.
//! - Data `x`, 48 bits.
//! 
//! **Note:** Is is *highly* recommended to always refer to these functions via the module; ie: `cell::XXX`.

use super::{cons::*, IntoRawBits64};
use core::num::NonZeroU64;

/// Indicates that the value is a cell.
pub const CELL_MARKER_BITS: u64 = SIGN_BIT | NAN_BITS;

/// Masks the bits ([`CELL_MARKER_BITS`]) that indicate that the value is a cell.
pub const CELL_MARKER_MASK: u64 = SIGN_BIT | NAN_BITS;

/// Masks out the tag of a [`CELL_MARKER_BITS`]-marked value.
pub const CELL_TAG_BITS: u64 = 0x0007000000000000;

/// Represents all possible variants for a cell-values 3-bit tag.
/// 
/// **Note:**  
/// Tag `0` is intentionally left undefined,
/// to prevent the value ever accidentally
/// becoming the original/sentinel `NaN`.
#[repr(u64)]
pub enum CellTag {
    // Tag0 is intentionally undefined.
    
    /// Tag `0b001`.
    Tag1 = 0x0001000000000000,
    
    /// Tag `0b010`.
    Tag2 = 0x0002000000000000,
    
    /// Tag `0b011`.
    Tag3 = 0x0003000000000000,
    
    /// Tag `0b100`.
    Tag4 = 0x0004000000000000,
    
    /// Tag `0b101`.
    Tag5 = 0x0005000000000000,
    
    /// Tag `0b110`.
    Tag6 = 0x0006000000000000,
    
    /// Tag `0b111`.
    Tag7 = 0x0007000000000000,
}

impl TryFrom<u64> for CellTag {
    type Error = (); // error left as unit type
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(match value {
            0x0001000000000000 => Self::Tag1,
            0x0002000000000000 => Self::Tag2,
            0x0003000000000000 => Self::Tag3,
            0x0004000000000000 => Self::Tag4,
            0x0005000000000000 => Self::Tag5,
            0x0006000000000000 => Self::Tag6,
            0x0007000000000000 => Self::Tag7,
            _ => return Err(())
        })
    }
}

// All cell-value tag variants, but as constants:

/// Cell Tag `0b001`.
pub const CELL_TAG_1: u64 = CellTag::Tag1 as u64;

/// Cell Tag `0b010`.
pub const CELL_TAG_2: u64 = CellTag::Tag2 as u64;

/// Cell Tag `0b011`.
pub const CELL_TAG_3: u64 = CellTag::Tag3 as u64;

/// Cell Tag `0b100`.
pub const CELL_TAG_4: u64 = CellTag::Tag4 as u64;

/// Cell Tag `0b101`.
pub const CELL_TAG_5: u64 = CellTag::Tag5 as u64;

/// Cell Tag `0b110`.
pub const CELL_TAG_6: u64 = CellTag::Tag6 as u64;

/// Cell Tag `0b111`.
pub const CELL_TAG_7: u64 = CellTag::Tag7 as u64;

/// Masks out the data of a [`CELL_MARKER_BITS`]-marked value.
pub const CELL_DATA_BITS: u64 = !(CELL_MARKER_BITS | CELL_TAG_BITS);

/// Ensure that the bit-patterns do not overlap.
#[test]
pub fn test_cell_bits() {
    assert!(CELL_MARKER_BITS != CELL_TAG_BITS);
    assert!(CELL_MARKER_BITS != CELL_DATA_BITS);
    assert!(CELL_TAG_BITS != CELL_DATA_BITS);
    assert!(CELL_MARKER_BITS & CELL_TAG_BITS & CELL_DATA_BITS == 0);
    assert!(CELL_MARKER_BITS | CELL_TAG_BITS | CELL_DATA_BITS == u64::MAX);
    assert!(CELL_MARKER_BITS ^ CELL_TAG_BITS ^ CELL_DATA_BITS == u64::MAX);
}

#[test]
#[cfg(feature = "std")]
pub fn test_ptr_roundtrip() {
    let val: &'static [u8] = &[0,1,2,3,4,5,6,7,8,9];
    let ptr_before = val.as_ptr() as *const ();
    let nan = from_tag_and_pointer(CellTag::Tag5, ptr_before).unwrap();
    let ptr_after = unwrap_cell_rawptr(nan).unwrap();
    assert!(ptr_before == ptr_after, "Before {ptr_before:?} == After {ptr_after:?}")
}

/// Returns wether the given value is a cell.
#[inline(always)]
pub fn is_cell(value: impl IntoRawBits64) -> bool {
    (value.as_raw_bits_64() & CELL_MARKER_BITS) == CELL_MARKER_BITS
}

/// Returns wether the given value is *not* a cell.
#[inline(always)]
pub fn is_not_cell(value: impl IntoRawBits64) -> bool {
    (value.as_raw_bits_64() & CELL_MARKER_BITS) != CELL_MARKER_BITS
}

/// Returns the tag bits of the given value.
#[inline(always)]
pub fn unwrap_tag_bits_unchecked(value: impl IntoRawBits64) -> u64 {
    value.as_raw_bits_64() & CELL_TAG_BITS
}

/// Returns the tag bits of the given value, if it is a cell.
#[inline(always)]
pub fn unwrap_tag_bits(value: impl IntoRawBits64) -> Option<u64> {
    match is_cell(value) {
        true => Some(value.as_raw_bits_64() & CELL_TAG_BITS),
        false => None
    }
}

/// Returns the tag bits of the given value, if it is a cell.
#[inline(always)]
pub fn unwrap_tag(value: impl IntoRawBits64) -> Option<CellTag> {
    match is_cell(value) {
        true => CellTag::try_from(value.as_raw_bits_64()).ok(),
        false => None
    }
}

/// Unwraps the cell-data of the given value as [`u64`], without checking if it is a cell.
#[inline(always)]
pub fn unwrap_cell_unchecked(value: impl IntoRawBits64) -> u64 {
    value.as_raw_bits_64() & CELL_DATA_BITS
}

/// Unwraps the cell-data of the given value as [`u64`], if it is a cell.
#[inline(always)]
pub fn unwrap_cell(value: impl IntoRawBits64) -> Option<u64> {
    match is_cell(value) {
        true => Some(unwrap_cell_unchecked(value)),
        false => None
    }
}

/// Unwraps the cell-data of the given value as [`NonZeroU64`], if it is a cell.
#[inline(always)]
pub fn unwrap_cell_nonzero(value: impl IntoRawBits64) -> Option<NonZeroU64> {
    match is_cell(value) {
        true => NonZeroU64::new(unwrap_cell_unchecked(value)),
        false => None
    }
}

/// Unwraps the cell-data of the given value as `*const ()`, if it is a cell.
/// 
/// # Safety
/// This function cannot check if the returned pointer is valid.
#[inline(always)]
pub fn unwrap_cell_rawptr(value: impl IntoRawBits64) -> Option<*const ()> {
    match is_cell(value) {
        true => Some(unwrap_cell_unchecked(value) as *const ()),
        false => None
    }
}

/// Combines the given tag and pointer into a NaN-tagged value.
/// 
/// If either the `tag` or the `ptr` don't fit in the limits
/// imposed by [`CELL_TAG_BITS`] and [`CELL_DATA_BITS`],
/// this function will return `None`.
/// 
/// # Safety
/// 
/// It is guaranteed that, if this function returns a `Some`-wrapped value,
/// that this value can be safely unwrapped via [`unwrap_cell_rawptr`]
/// yielding exactly the same pointer as it was created from.
/// 
/// However, performing any kind of logic- or arithmetic-operations
/// on the returned value, will result in undefined behaviour.
pub fn from_tag_and_pointer(tag: CellTag, ptr: *const ()) -> Option<u64> {
    //let ptr: *const () = ptr.into();
    let vtag = (tag as u64) & CELL_TAG_BITS;
    let vptr = (ptr as u64) & CELL_DATA_BITS;
    if vtag != tag as u64 {return None}
    if vptr != ptr as u64 {return None}
    Some(CELL_MARKER_BITS | vtag | vptr)
}
