# nanval

[![Crates.io](https://img.shields.io/crates/v/nanval?style=flat)](https://crates.io/crates/nanval)
[![Docs.rs](https://docs.rs/nanval/badge.svg)](https://docs.rs/nanval)
[![GitHub](https://img.shields.io/github/languages/top/Longor1996/nanval)](https://github.com/Longor1996/nanval)
[![LOC](https://tokei.rs/b1/github/Longor1996/nanval)](https://github.com/Longor1996/nanval)

A `no_std`, zero-dependency crate for the creation and handling of NaN-tagged 64-bit floating-point values.

> Inspired by [this article](https://sean.cm/a/nan-boxing) and [this crate](https://github.com/Marwes/nanbox).

## How does this work?

When a 64-bit floating-point number is set to `NaN`/`0x7FF8000000000000`, its bits are as follows:

```text
s111 1111 1111 1qxx xxxx xxxx xxxx xxxx xxxx xxxx xxxx xxxx xxxx xxxx xxxx xxxx
^               ^\____________________________________________________________/
|               |                             ^
| Sign Bit      | Quiet Bit                   | Data Bits
```

As long as the data bits aren't all set to `0`, indicating the original/sentinel `NaN` value, they can be literally anything else! This gives us 50 bits to mess with/use as we please...

### UInts / Unsigned Integers

> Look at the module [`crate::uint`] for this.

**TODO:** Add explanation.

### Cells / Pointers

> Look at the module [`crate::cell`] for this.

Since it doesn't matter what the sign-bit `s` is set to, we can use it as a flag/marker that indicates that the value is some kind of `cell` or `ptr`.

Combine this with the fact that basically all x64-platforms only use the lower 48 or 50 bits for addressing (ignoring CHERI shenanigans), we are left with 3 bits (that includes the 'quiet' bit) to store some kind of type-tag for the cell; look at the [`crate::cell::CellTag`].

## References

- <https://float.exposed/0x7ff8000000000000>
- <https://sean.cm/a/nan-boxing>
- <https://github.com/Marwes/nanbox/blob/master/src/lib.rs>
- <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibJS/Runtime/Value.h>
