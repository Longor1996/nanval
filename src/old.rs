use super::*;
use core::num::NonZeroU16;

const TAG_SHIFT: u64 = 48;
const TAG_MASK:   u64 = 0x0000FFFF00000000;
//const TAG_UNMASK: u64 = 0xFFFF0000FFFFFFFF;

const DAT_MASK:   u64 = 0x00000000FFFFFFFF;
const DAT_UNMASK: u64 = 0xFFFFFFFF00000000;

/// A tag that can be converted from/into a `NonZeroU16`.
pub trait NaNTag: From<NonZeroU16> + Into<NonZeroU16> + Clone + Copy {}
impl NaNTag for NonZeroU16 {}

/// Data that can be converted from/into a `u32`.
pub trait NaNDat: From<u32> + Into<u32> + Clone + Copy {}
impl NaNDat for u32 {}

/// A raw NaN-tagged value.
#[repr(C)]
#[derive(Clone, Copy)]
pub union RawNaNVal<TAG, DAT> where TAG: NaNTag, DAT: NaNDat {
    f: f64,
    u: u64,
    p: core::marker::PhantomData<(TAG,DAT)>
}

impl<TAG: NaNTag, DAT: NaNDat> RawNaNVal<TAG, DAT> {
    /// Creates a new [`RawNaNVal`] from the given `f64`, if valid.
    pub fn from_float(f: f64) -> Option<Self> {
        let new = Self {f};
        if new.has_tag() {
            return None
        }
        Some(new)
    }
    
    /// Creates a new [`RawNaNVal`] from the given `f64`.
    /// 
    /// # Safety
    /// If there is a tag set, the returned [`RawNaNVal`] will have an invalid tag and data.
    pub fn from_float_unchecked(f: f64) -> Self {
        let new = Self {f};
        debug_assert!(!new.has_tag(), "a float must not have tag");
        new
    }
    
    /// Creates a new [`RawNaNVal`] from the given tag and no data.
    pub fn from_tag(tag: TAG) -> Self {
        let tag: u16 = tag.into().get();
        let tag = (tag as u64) << TAG_SHIFT;
        let tag = tag & TAG_MASK; // no spilling
        Self {u: NAN_SIGNAL | tag}
    }
    
    /// Creates a new [`RawNaNVal`] from the given tag and data.
    pub fn from_tag_and_data(tag: TAG, dat: DAT) -> Self {
        let tag: u16 = tag.into().get();
        let tag = (tag as u64) << TAG_SHIFT;
        let tag = tag & TAG_MASK; // no spilling
        
        let dat: u32 = dat.into();
        let dat = dat as u64;
        let dat = dat & DAT_MASK; // no spilling
        
        Self {u: NAN_SIGNAL | tag | dat}
    }
    
    /// Returns the tag, ignoring the signal
    /// 
    /// # Safety
    /// If the tag is not set (use `has_tag` to check), undefined behaviour will occur.
    pub unsafe fn get_tag_unchecked(&self) -> TAG {
        debug_assert!(self.has_tag(), "invalid tag");
        let tag = (self.u & TAG_MASK) >> TAG_SHIFT;
        NonZeroU16::new_unchecked(tag as u16).into()
    }
    
    /// Returns the data, ignoring the signal
    /// 
    /// # Safety
    /// You can (and will) receive corrupted data if the `has_tag`-check is skipped.
    pub fn get_dat_raw_unchecked(&self) -> DAT {
        debug_assert!(self.has_tag(), "invalid tag");
        unsafe {((self.u & DAT_MASK) as u32).into()}
    }
    
    /// Returns if `self` is a tag & data.
    pub fn has_tag(&self) -> bool {
        unsafe {
            (self.u & NAN_MASK) == NAN_SIGNAL
        }
    }
    
    /// Returns if `self` is a `f64`.
    pub fn has_f64(&self) -> bool {
        unsafe {
            (self.u & NAN_MASK) != NAN_SIGNAL
        }
    }
    
    /// Overrides `self`s tag with the given tag.
    /// 
    /// # Safety
    /// You can (and will) receive corrupted data if the `has_tag`-check is skipped.
    pub fn set_tag_unchecked(&mut self, tag: TAG) {
        debug_assert!(self.has_tag(), "retagging a float is invalid");
        let tag: u16 = tag.into().get();
        let tag = (tag as u64) << TAG_SHIFT;
        let tag = tag & TAG_MASK; // no spilling
        unsafe {
            self.u &= NAN_UNMASK; // remove NaN and tag
            self.u |= NAN_SIGNAL | tag; // insert NaN and tag
        }
    }
    
    /// Overrides `self`s data with the given data.
    /// 
    /// # Safety
    /// You can (and will) receive corrupted data if the `has_tag`-check is skipped.
    pub fn set_dat_unchecked(&mut self, dat: DAT) {
        debug_assert!(self.has_tag(), "partially rewriting a float is invalid");
        let dat: u32 = dat.into();
        let dat = dat as u64;
        let dat = dat & DAT_MASK; // no spilling
        unsafe {
            self.u &= DAT_UNMASK; // remove dat
            self.u |= dat; // insert dat
        }
    }
    
    /// Overrides `self`s tag and data with the given values.
    pub fn set_tag_and_dat(&mut self, tag: TAG, dat: DAT) {
        let tag: u16 = tag.into().get();
        let tag = (tag as u64) << TAG_SHIFT;
        let tag = tag & TAG_MASK; // no spilling
        
        let dat: u32 = dat.into();
        let dat = dat as u64;
        let dat = dat & DAT_MASK; // no spilling
        
        // This is locally safe, as the data is fully overwritten.
        self.u = NAN_SIGNAL | tag | dat;
    }
    
    /// Tries to create a copy of `self`, with the given `DAT`, using the existing `TAG`.
    pub fn with_dat(&self, dat: DAT) -> Option<Self> {
        if self.has_tag() {
            let mut n = *self;
            n.set_dat_unchecked(dat);
            return Some(n)
        }
        
        None
    }
    
    /// Replaces `self` with the given `f64`-value.
    pub fn set_f64(&mut self, f: f64) {
        // This is locally safe, as the data is fully overwritten.
        self.f = f;
    }
    
    /// Returns the contained `f64`-value, or `None`.
    pub fn get_f64(&self) -> Option<f64> {
        if self.has_tag() {
            None
        } else {
            // We just checked that there is no tag, so this is safe.
            Some(unsafe {self.f})
        }
    }
    
    /// Returns the tag, or `None`.
    pub fn get_tag(&self) -> Option<TAG> {
        if self.has_tag() {
            // We just checked that there is a tag, so this is safe.
            let tag = unsafe {self.get_tag_unchecked()};
            Some(tag)
        } else {
            None
        }
    }
    
    /// Returns the data, or `None`.
    pub fn get_dat(&self) -> Option<DAT> {
        if self.has_tag() {
            // We just checked that there is a tag, so this is safe.
            Some(self.get_dat_raw_unchecked())
        } else {
            None
        }
    }
    
    /// Returns the tag and data, or `None`.
    pub fn get_tag_and_dat(&self) -> Option<(TAG, DAT)> {
        if self.has_tag() {
            // We just checked that there is a tag, so this is safe.
            Some((
                unsafe{self.get_tag_unchecked()},
                self.get_dat_raw_unchecked()
            ))
        } else {
            None
        }
    }
}

// The following impl's are always safe.

impl<TAG: NaNTag, DAT: NaNDat> TryFrom<RawNaNVal<TAG, DAT>> for f64 {
    type Error = ();

    fn try_from(value: RawNaNVal<TAG, DAT>) -> Result<Self, Self::Error> {
        value.get_f64().ok_or(())
    }
}

impl<TAG: NaNTag, DAT: NaNDat> TryFrom<f64> for RawNaNVal<TAG, DAT> {
    type Error = ();
    
    fn try_from(f: f64) -> Result<Self, Self::Error> {
        let new = Self {f};
        new.has_tag().then(||new).ok_or(())
    }
}

impl<TAG: NaNTag, DAT: NaNDat> From<TAG> for RawNaNVal<TAG, DAT> {
    fn from(tag: TAG) -> Self {
        Self::from_tag(tag)
    }
}

impl<TAG: NaNTag, DAT: NaNDat> From<(TAG, DAT)> for RawNaNVal<TAG, DAT> {
    fn from((tag, dat): (TAG, DAT)) -> Self {
        Self::from_tag_and_data(tag, dat)
    }
}

#[test]
fn size() {
    assert!(core::mem::size_of::<RawNaNVal<NonZeroU16, u32>>() == 8)
}